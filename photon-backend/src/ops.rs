//! Photon IO + mapping for ops UI reads (feature `ops`).
//!
//! Server functions in `photon-app` resolve session / Photon context, then call
//! these helpers so SSR and IsolatedLab tests can exercise the same path without
//! Leptos request plumbing.

use std::sync::Arc;

use photon::Photon;

use crate::{
    clamp_event_list_limit, count_since, dashboard_stats, event_detail_from_transport,
    event_summary_from_transport, find_checkpoint_seq, find_subscription_by_id, find_topic_by_name,
    sort_topics_by_name, subscription_summary_from_handler, topic_summary, validate_event_id,
    validate_subscription_id, validate_topic_name, DashboardStats, EventDetail, EventSummary,
    PhotonIdError, SubscriptionSummary, TopicSummary,
};

/// Operator-facing ops failure (maps to `ServerFnError` at the Leptos boundary).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpsError {
    /// Missing authenticated session.
    AuthenticationRequired,
    /// Photon was not provided in request / lab context.
    PhotonNotInContext,
    /// Permission check failed (`PhotonAdmin`).
    PermissionDenied,
    /// Path/query id failed validation.
    InvalidId(PhotonIdError),
    /// Photon IO or snapshot failure.
    Photon(String),
}

impl std::fmt::Display for OpsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticationRequired => write!(f, "Authentication required"),
            Self::PhotonNotInContext => write!(f, "Photon not in request context"),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::InvalidId(e) => write!(f, "{e}"),
            Self::Photon(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for OpsError {}

impl From<PhotonIdError> for OpsError {
    fn from(value: PhotonIdError) -> Self {
        Self::InvalidId(value)
    }
}

/// Reject anonymous callers (session user id absent).
pub fn require_session_user(user_id: Option<&str>) -> Result<(), OpsError> {
    if user_id.is_some() {
        Ok(())
    } else {
        Err(OpsError::AuthenticationRequired)
    }
}

/// Reject callers without `PhotonAdmin` (lab / integ seam; server macro also gates).
pub fn require_photon_admin(has_admin: bool) -> Result<(), OpsError> {
    if has_admin {
        Ok(())
    } else {
        Err(OpsError::PermissionDenied)
    }
}

/// Require a Photon handle (mirrors Leptos context lookup).
pub fn require_photon(photon: Option<Arc<Photon>>) -> Result<Arc<Photon>, OpsError> {
    photon.ok_or(OpsError::PhotonNotInContext)
}

fn map_transport_event(ev: &photon::Event) -> EventSummary {
    event_summary_from_transport(
        ev.event_id.clone(),
        ev.topic_name.clone(),
        ev.topic_key.clone(),
        ev.seq,
        ev.created_at.to_rfc3339(),
    )
}

/// Handler inventory + checkpoints via `admin_snapshot`.
pub async fn list_subscriptions(photon: &Photon) -> Result<Vec<SubscriptionSummary>, OpsError> {
    let snap = photon
        .admin_snapshot()
        .await
        .map_err(|e| OpsError::Photon(format!("Failed to load admin snapshot: {e}")))?;
    let checkpoints: Vec<(String, String, Option<String>, Option<i64>)> = snap
        .checkpoints
        .iter()
        .map(|c| {
            (
                c.subscription_name.clone(),
                c.topic_name.clone(),
                c.topic_key.clone(),
                c.last_seq,
            )
        })
        .collect();

    let mut list = Vec::with_capacity(snap.handlers.len());
    for h in snap.handlers {
        let last_seq =
            find_checkpoint_seq(&checkpoints, h.subscription_name.as_deref(), &h.topic_name);
        list.push(subscription_summary_from_handler(
            h.registry_key,
            h.subscription_name.or(h.consumer_group),
            h.topic_name,
            h.mode,
            last_seq,
        ));
    }
    Ok(list)
}

/// Dashboard KPIs from registry size, subscription count, and 24h events.
pub async fn load_dashboard_stats(photon: &Photon) -> Result<DashboardStats, OpsError> {
    let topic_count = photon.registry().len() as u32;
    let subscription_count = list_subscriptions(photon).await?.len() as u32;

    let events = photon
        .list_recent_events(clamp_event_list_limit(1000))
        .await
        .map_err(|e| OpsError::Photon(format!("Failed to list recent events: {e}")))?;
    let now = chrono::Utc::now();
    let day_ago = now - chrono::Duration::hours(24);
    let stamps: Vec<_> = events.iter().map(|e| e.created_at).collect();
    let event_count_24h = count_since(&stamps, day_ago);

    Ok(dashboard_stats(
        topic_count,
        subscription_count,
        event_count_24h,
    ))
}

/// Recent events for the dashboard preview table.
pub async fn load_recent_events(
    photon: &Photon,
    limit: u32,
) -> Result<Vec<EventSummary>, OpsError> {
    let limit = clamp_event_list_limit(limit);
    let events = photon
        .list_recent_events(limit)
        .await
        .map_err(|e| OpsError::Photon(format!("Failed to list recent events: {e}")))?;
    Ok(events.iter().map(map_transport_event).collect())
}

/// All registry topics with per-topic subscription and 24h event counts.
pub async fn load_topics(photon: &Photon) -> Result<Vec<TopicSummary>, OpsError> {
    let subs = list_subscriptions(photon).await?;
    let registry = photon.registry();
    let mut topics = Vec::new();
    for desc in registry.iter() {
        let topic_name = desc.topic_name.to_string();
        let subscription_count = subs.iter().filter(|s| s.topic_name == topic_name).count() as u32;

        let events = photon
            .list_events_by_topic(&topic_name, None, None, clamp_event_list_limit(1000))
            .await
            .map_err(|e| OpsError::Photon(format!("Failed to list events: {e}")))?;
        let now = chrono::Utc::now();
        let day_ago = now - chrono::Duration::hours(24);
        let stamps: Vec<_> = events.iter().map(|e| e.created_at).collect();
        let event_count_24h = count_since(&stamps, day_ago);

        topics.push(topic_summary(
            topic_name,
            desc.keyed_by.map(|s| s.to_string()),
            desc.schema_json.to_string(),
            event_count_24h,
            subscription_count,
        ));
    }
    sort_topics_by_name(&mut topics);
    Ok(topics)
}

/// Single topic by validated name.
pub async fn load_topic(
    photon: &Photon,
    topic_name: &str,
) -> Result<Option<TopicSummary>, OpsError> {
    validate_topic_name(topic_name)?;
    let topics = load_topics(photon).await?;
    Ok(find_topic_by_name(&topics, topic_name).cloned())
}

/// Single subscription by validated id.
pub async fn load_subscription(
    photon: &Photon,
    id: &str,
) -> Result<Option<SubscriptionSummary>, OpsError> {
    validate_subscription_id(id)?;
    let subs = list_subscriptions(photon).await?;
    Ok(find_subscription_by_id(&subs, id).cloned())
}

/// Events across all topics or filtered by topic name.
pub async fn load_events(
    photon: &Photon,
    topic_name: Option<&str>,
    limit: u32,
) -> Result<Vec<EventSummary>, OpsError> {
    let limit = clamp_event_list_limit(limit);
    let events = if let Some(topic) = topic_name {
        validate_topic_name(topic)?;
        photon
            .list_events_by_topic(topic, None, None, limit)
            .await
            .map_err(|e| OpsError::Photon(format!("Failed to list events: {e}")))?
    } else {
        photon
            .list_recent_events(limit)
            .await
            .map_err(|e| OpsError::Photon(format!("Failed to list recent events: {e}")))?
    };
    Ok(events.iter().map(map_transport_event).collect())
}

/// Single event detail by validated id.
pub async fn load_event(photon: &Photon, id: &str) -> Result<Option<EventDetail>, OpsError> {
    validate_event_id(id)?;
    let transport = photon
        .get_event(id)
        .await
        .map_err(|e| OpsError::Photon(format!("Failed to get transport event: {e}")))?;

    Ok(transport.map(|t| {
        event_detail_from_transport(
            t.event_id,
            t.topic_name,
            t.topic_key,
            t.seq,
            t.created_at.to_rfc3339(),
            t.payload_json,
            t.actor_json,
        )
    }))
}
