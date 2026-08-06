//! Leptos server functions for Photon UI.
//!
//! DTOs and pure mapping helpers live in [`photon_backend`] so contracts stay
//! unit/integration-testable without the host UI graph. Server functions run on
//! SSR only and use Photon request context for IO (Chronon/Boson-shaped: no
//! Valence ops projection).
//!
//! ## Security map
//!
//! - Every ops-UI server fn requires an authenticated session and
//!   `PhotonAdmin` (via `#[uf_product_macros::server(permission = "...")]`).
//! - Catalog, subscription, and event reads come from Photon (`admin_snapshot`,
//!   registry, `list_*`, `get_event`).

use leptos::prelude::*;
pub use photon_backend::{
    DashboardStats, EventDetail, EventSummary, SubscriptionSummary, TopicSummary,
};

/// Permission name required for Photon admin reads
/// (manifest: [`crate::permissions::PhotonPermission::PhotonAdmin`]).
pub const PHOTON_ADMIN_PERMISSION: &str = "PhotonAdmin";

#[cfg(feature = "ssr")]
use photon_backend::{
    clamp_event_list_limit, count_since, dashboard_stats, event_detail_from_transport,
    event_summary_from_transport, find_checkpoint_seq, find_subscription_by_id, find_topic_by_name,
    sort_topics_by_name, subscription_summary_from_handler, topic_summary, validate_event_id,
    validate_subscription_id, validate_topic_name,
};

#[cfg(feature = "ssr")]
fn photon_from_context() -> Result<std::sync::Arc<photon::Photon>, ServerFnError> {
    leptos::context::use_context::<std::sync::Arc<photon::Photon>>()
        .ok_or_else(|| ServerFnError::new("Photon not in request context"))
}

#[cfg(feature = "ssr")]
fn require_session(ctx: &higgs::Higgs) -> Result<(), ServerFnError> {
    if ctx.session_user_id().is_some() {
        Ok(())
    } else {
        Err(ServerFnError::new("Authentication required"))
    }
}

#[cfg(feature = "ssr")]
fn map_transport_event(ev: &photon::Event) -> EventSummary {
    event_summary_from_transport(
        ev.event_id.clone(),
        ev.topic_name.clone(),
        ev.topic_key.clone(),
        ev.seq,
        ev.created_at.to_rfc3339(),
    )
}

#[cfg(feature = "ssr")]
async fn subscriptions_from_photon(
    photon: &photon::Photon,
) -> Result<Vec<SubscriptionSummary>, ServerFnError> {
    let snap = photon
        .admin_snapshot()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to load admin snapshot: {e}")))?;
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
        let last_seq = find_checkpoint_seq(
            &checkpoints,
            h.subscription_name.as_deref(),
            &h.topic_name,
        );
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

// ============================================================================
// Server functions (SSR only)
// ============================================================================

/// Get dashboard stats (topic count, subscription count, event count 24h).
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;

    let topic_count = photon.registry().len() as u32;
    let subscription_count = subscriptions_from_photon(&photon).await?.len() as u32;

    let events = photon
        .list_recent_events(clamp_event_list_limit(1000))
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {e}")))?;
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

/// Get recent events for dashboard.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_recent_events(
    /// Maximum number of recent events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    let limit = clamp_event_list_limit(limit);

    let events = photon
        .list_recent_events(limit)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {e}")))?;

    Ok(events.iter().map(map_transport_event).collect())
}

/// Get all topics (from registry + Photon list counts).
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_topics() -> Result<Vec<TopicSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;

    let subs = subscriptions_from_photon(&photon).await?;
    let registry = photon.registry();
    let mut topics = Vec::new();
    for desc in registry.iter() {
        let topic_name = desc.topic_name.to_string();
        let subscription_count = subs
            .iter()
            .filter(|s| s.topic_name == topic_name)
            .count() as u32;

        let events = photon
            .list_events_by_topic(&topic_name, None, None, clamp_event_list_limit(1000))
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list events: {e}")))?;
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

/// Get a single topic by name.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_topic(
    /// Name of the topic to look up.
    topic_name: String,
) -> Result<Option<TopicSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    validate_topic_name(&topic_name).map_err(ServerFnError::new)?;
    let topics = get_topics().await?;
    Ok(find_topic_by_name(&topics, &topic_name).cloned())
}

/// Get all subscriptions (handler inventory + checkpoints via `admin_snapshot`).
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_subscriptions() -> Result<Vec<SubscriptionSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    subscriptions_from_photon(&photon).await
}

/// Get a single subscription by ID.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_subscription(
    /// Unique identifier of the subscription to look up.
    id: String,
) -> Result<Option<SubscriptionSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    validate_subscription_id(&id).map_err(ServerFnError::new)?;
    let subs = get_subscriptions().await?;
    Ok(find_subscription_by_id(&subs, &id).cloned())
}

/// Get events (recent across all topics, or optionally filter by topic).
///
/// Results are capped at [`photon_backend::MAX_EVENT_LIST_LIMIT`]. When
/// `topic_name` is set, the query is topic-scoped via Photon `list_events_by_topic`.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_events(
    /// Optional topic name to restrict results to; when omitted, events from all
    /// topics are considered (still capped).
    topic_name: Option<String>,
    /// Maximum number of events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    let limit = clamp_event_list_limit(limit);

    let events = if let Some(topic) = &topic_name {
        validate_topic_name(topic).map_err(ServerFnError::new)?;
        photon
            .list_events_by_topic(topic, None, None, limit)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list events: {e}")))?
    } else {
        photon
            .list_recent_events(limit)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {e}")))?
    };

    Ok(events.iter().map(map_transport_event).collect())
}

/// Get a single event by ID.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_event(
    /// Unique identifier of the event to look up.
    id: String,
) -> Result<Option<EventDetail>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    validate_event_id(&id).map_err(ServerFnError::new)?;
    let photon = photon_from_context()?;

    let transport = photon
        .get_event(&id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get transport event: {e}")))?;

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
