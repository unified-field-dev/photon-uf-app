//! Leptos server functions and DTOs for Photon UI.
//!
//! Types are used by both server and client. Server functions run on SSR only
//! and use `higgs::Higgs::from_request()` for unified context extraction.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// DTOs (used by server functions and UI)
// ============================================================================

/// Summary of a Photon topic for list/detail views: registry metadata plus
/// recent traffic counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicSummary {
    /// Topic name as registered with Photon.
    pub topic_name: String,
    /// Optional key field the topic is partitioned/keyed by.
    pub keyed_by: Option<String>,
    /// JSON-encoded schema description for the topic's payload.
    pub schema_json: String,
    /// Number of events published to this topic in the last 24 hours.
    pub event_count_24h: u64,
    /// Number of subscriptions currently registered against this topic.
    pub subscription_count: u32,
}

/// Summary of a Photon subscription for list/detail views: configuration
/// plus read-state visibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionSummary {
    /// Unique subscription identifier.
    pub subscription_id: String,
    /// Human-readable subscription name.
    pub subscription_name: String,
    /// Name of the topic this subscription is attached to.
    pub topic_name: String,
    /// Whether the subscription is currently enabled.
    pub enabled: bool,
    /// Dispatch mode (e.g. at-least-once, durable) as a display string.
    pub mode: String,
    /// Optional topic key filter restricting which keys this subscription receives.
    pub topic_key_filter: Option<String>,
    /// Number of events the subscription's checkpoint is behind the topic head.
    pub checkpoint_lag: i64,
    /// Sequence number of the last event this subscription has processed, if any.
    pub last_seq: Option<i64>,
    /// Timestamp of the last successfully processed event, if any.
    pub last_processed_at: Option<String>,
}

/// Summary of a single Photon event for list views and dashboards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventSummary {
    /// Unique event identifier.
    pub event_id: String,
    /// Name of the topic the event was published to.
    pub topic_name: String,
    /// Optional key the event was published under.
    pub topic_key: Option<String>,
    /// Sequence number of the event within its topic.
    pub seq: i64,
    /// RFC3339 timestamp of when the event was created.
    pub created_at: String,
    /// Short, human-readable preview of the event's delivery status/payload.
    pub payload_preview: String,
}

/// Aggregate counters shown on the Photon dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    /// Total number of registered topics.
    pub topic_count: u32,
    /// Total number of registered subscriptions.
    pub subscription_count: u32,
    /// Number of events published across all topics in the last 24 hours.
    pub event_count_24h: u64,
}

/// Full detail record for a single event, including payload and actor context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDetail {
    /// Unique event identifier.
    pub event_id: String,
    /// Name of the topic the event was published to.
    pub topic_name: String,
    /// Optional key the event was published under.
    pub topic_key: Option<String>,
    /// Sequence number of the event within its topic.
    pub seq: i64,
    /// RFC3339 timestamp of when the event was created.
    pub created_at: String,
    /// Delivery status recorded for the event (e.g. delivered, dead-lettered).
    pub delivery_status: String,
    /// Raw event payload JSON, if the transport record is still available.
    pub payload_json: serde_json::Value,
    /// Raw actor JSON captured at publish time, if the transport record is still available.
    pub actor_json: serde_json::Value,
    /// True when Valence metadata exists but continuum transport payload is gone.
    pub transport_expired: bool,
}

#[cfg(feature = "ssr")]
fn photon_from_context() -> Result<std::sync::Arc<photon::Photon>, ServerFnError> {
    use leptos::prelude::*;
    leptos::context::use_context::<std::sync::Arc<photon::Photon>>()
        .ok_or_else(|| ServerFnError::new("Photon not in request context"))
}

#[cfg(feature = "ssr")]
async fn system_valence() -> Result<valence::Valence, ServerFnError> {
    uf_ssr::ssr::system_valence().await
}

#[cfg(feature = "ssr")]
fn map_event_summary(e: photon_valence_admin::persistence::DbEvent) -> EventSummary {
    EventSummary {
        event_id: e.id().map(|x| x.to_string()).unwrap_or_default(),
        topic_name: e.topic_name().to_string(),
        topic_key: e.topic_key().map(|s| s.to_string()),
        seq: *e.seq(),
        created_at: e.created_at().to_rfc3339(),
        payload_preview: format!("[{}]", e.delivery_status()),
    }
}

// ============================================================================
// Server functions (SSR only)
// ============================================================================

/// Get dashboard stats (topic count, subscription count, event count 24h).
#[uf_product_macros::server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let photon = photon_from_context()?;
    let valence = system_valence().await?;

    let registry = photon.registry();
    let topic_count = registry.len() as u32;

    let subs = photon_valence_admin::persistence::SubscriptionStore::list(&valence)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list subscriptions: {}", e)))?;
    let subscription_count = subs.len() as u32;

    let events = photon_valence_admin::persistence::EventStore::list_recent(&valence, 1000)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {}", e)))?;
    let now = chrono::Utc::now();
    let day_ago = now - chrono::Duration::hours(24);
    let event_count_24h = events.iter().filter(|e| *e.created_at() >= day_ago).count() as u64;

    Ok(DashboardStats {
        topic_count,
        subscription_count,
        event_count_24h,
    })
}

/// Get recent events for dashboard.
#[uf_product_macros::server]
pub async fn get_recent_events(
    /// Maximum number of recent events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let valence = system_valence().await?;

    let events = photon_valence_admin::persistence::EventStore::list_recent(&valence, limit as usize)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {}", e)))?;

    let list: Vec<EventSummary> = events.into_iter().map(map_event_summary).collect();
    Ok(list)
}

/// Get all topics (from registry + optional DB counts).
#[uf_product_macros::server]
pub async fn get_topics() -> Result<Vec<TopicSummary>, ServerFnError> {
    let photon = photon_from_context()?;
    let valence = system_valence().await?;

    let registry = photon.registry();
    let mut topics = Vec::new();
    for desc in registry.iter() {
        let topic_name = desc.topic_name.to_string();
        let subscription_count =
            photon_valence_admin::persistence::SubscriptionStore::list_by_topic(&valence, &topic_name)
                .await
                .map_err(|e| ServerFnError::new(format!("Failed to list subscriptions: {}", e)))?
                .len() as u32;

        let events = photon_valence_admin::persistence::EventStore::list_recent(&valence, 10000)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list events: {}", e)))?;
        let now = chrono::Utc::now();
        let day_ago = now - chrono::Duration::hours(24);
        let event_count_24h = events
            .iter()
            .filter(|e| e.topic_name().as_str() == topic_name)
            .filter(|e| *e.created_at() >= day_ago)
            .count() as u64;

        topics.push(TopicSummary {
            topic_name,
            keyed_by: desc.keyed_by.map(|s| s.to_string()),
            schema_json: desc.schema_json.to_string(),
            event_count_24h,
            subscription_count,
        });
    }
    topics.sort_by(|a, b| a.topic_name.cmp(&b.topic_name));
    Ok(topics)
}

/// Get a single topic by name.
#[uf_product_macros::server]
pub async fn get_topic(
    /// Name of the topic to look up.
    topic_name: String,
) -> Result<Option<TopicSummary>, ServerFnError> {
    let topics = get_topics().await?;
    Ok(topics.into_iter().find(|t| t.topic_name == topic_name))
}

/// Get all subscriptions.
#[uf_product_macros::server]
pub async fn get_subscriptions() -> Result<Vec<SubscriptionSummary>, ServerFnError> {
    let valence = system_valence().await?;

    let subs = photon_valence_admin::persistence::SubscriptionStore::list(&valence)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list subscriptions: {}", e)))?;

    let mut list = Vec::with_capacity(subs.len());
    for s in subs {
        let sub_id = s.id().map(|x| x.to_string()).unwrap_or_default();
        let topic_name = s.topic_name().to_string();
        let sub_name = s.subscription_name().to_string();
        let topic_key_filter = s.topic_key_filter().map(|x| x.to_string());
        let last_seq = s.get_checkpoints(&valence).await.ok().and_then(|cps| {
            cps.into_iter()
                .find(|c| {
                    *c.topic_name() == topic_name
                        && c.topic_key().map(|k| k.as_str()) == topic_key_filter.as_deref()
                })
                .map(|c| *c.last_seq())
        });
        let checkpoint_lag = 0i64;
        list.push(SubscriptionSummary {
            subscription_id: sub_id,
            subscription_name: sub_name,
            topic_name,
            enabled: *s.enabled(),
            mode: s.mode().to_string(),
            topic_key_filter,
            checkpoint_lag,
            last_seq,
            last_processed_at: None,
        });
    }
    Ok(list)
}

/// Get a single subscription by ID.
#[uf_product_macros::server]
pub async fn get_subscription(
    /// Unique identifier of the subscription to look up.
    id: String,
) -> Result<Option<SubscriptionSummary>, ServerFnError> {
    let subs = get_subscriptions().await?;
    Ok(subs.into_iter().find(|s| s.subscription_id == id))
}

/// Get events (recent across all topics, or optionally filter by topic).
#[uf_product_macros::server]
pub async fn get_events(
    /// Optional topic name to restrict results to; when omitted, events from all
    /// topics are considered.
    topic_name: Option<String>,
    /// Maximum number of events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let valence = system_valence().await?;

    let events = if let Some(topic) = &topic_name {
        photon_valence_admin::persistence::EventStore::list_by_topic(&valence, topic, None, None, limit as usize)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list events: {}", e)))?
    } else {
        photon_valence_admin::persistence::EventStore::list_recent(&valence, limit as usize)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {}", e)))?
    };

    let list: Vec<EventSummary> = events.into_iter().map(map_event_summary).collect();
    Ok(list)
}

/// Get a single event by ID.
#[uf_product_macros::server]
pub async fn get_event(
    /// Unique identifier of the event to look up.
    id: String,
) -> Result<Option<EventDetail>, ServerFnError> {
    let photon = photon_from_context()?;
    let valence = system_valence().await?;

    let meta = photon_valence_admin::persistence::EventStore::get_by_id(&valence, &id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to load projection: {}", e)))?;

    let Some(meta) = meta else {
        return Ok(None);
    };

    let transport = photon
        .get_event(&id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get transport event: {}", e)))?;

    let (payload_json, actor_json, transport_expired) = match transport {
        Some(t) => (t.payload_json, t.actor_json, false),
        None => (
            serde_json::Value::Null,
            serde_json::Value::Null,
            true,
        ),
    };

    Ok(Some(EventDetail {
        event_id: id,
        topic_name: meta.topic_name().to_string(),
        topic_key: meta.topic_key().map(|s| s.to_string()),
        seq: *meta.seq(),
        created_at: meta.created_at().to_rfc3339(),
        delivery_status: meta.delivery_status().to_string(),
        payload_json,
        actor_json,
        transport_expired,
    }))
}
