//! Leptos server functions for Photon UI.
//!
//! DTOs and pure mapping helpers live in [`photon_backend`] so contracts stay
//! unit/integration-testable without the host UI graph. Server functions run on
//! SSR only and use Photon / Valence request context for IO.

use leptos::prelude::*;
pub use photon_backend::{
    DashboardStats, EventDetail, EventSummary, SubscriptionSummary, TopicSummary,
};

#[cfg(feature = "ssr")]
use photon_backend::{
    count_since, dashboard_stats, event_detail_transport_expired, event_summary_from_meta,
    find_subscription_by_id, find_topic_by_name, sort_topics_by_name, stub_checkpoint_lag,
    validate_event_id, validate_subscription_id, validate_topic_name,
};

#[cfg(feature = "ssr")]
fn photon_from_context() -> Result<std::sync::Arc<photon::Photon>, ServerFnError> {
    leptos::context::use_context::<std::sync::Arc<photon::Photon>>()
        .ok_or_else(|| ServerFnError::new("Photon not in request context"))
}

#[cfg(feature = "ssr")]
async fn system_valence() -> Result<valence::Valence, ServerFnError> {
    uf_ssr::ssr::system_valence().await
}

#[cfg(feature = "ssr")]
fn map_event_summary(e: photon_valence_admin::persistence::DbEvent) -> EventSummary {
    event_summary_from_meta(
        e.id().map(|x| x.to_string()).unwrap_or_default(),
        e.topic_name().to_string(),
        e.topic_key().map(|s| s.to_string()),
        *e.seq(),
        e.created_at().to_rfc3339(),
        e.delivery_status(),
    )
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
        .map_err(|e| ServerFnError::new(format!("Failed to list subscriptions: {e}")))?;
    let subscription_count = subs.len() as u32;

    let events = photon_valence_admin::persistence::EventStore::list_recent(&valence, 1000)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {e}")))?;
    let now = chrono::Utc::now();
    let day_ago = now - chrono::Duration::hours(24);
    let stamps: Vec<_> = events.iter().map(|e| *e.created_at()).collect();
    let event_count_24h = count_since(&stamps, day_ago);

    Ok(dashboard_stats(
        topic_count,
        subscription_count,
        event_count_24h,
    ))
}

/// Get recent events for dashboard.
#[uf_product_macros::server]
pub async fn get_recent_events(
    /// Maximum number of recent events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let valence = system_valence().await?;

    let events =
        photon_valence_admin::persistence::EventStore::list_recent(&valence, limit as usize)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {e}")))?;

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
            photon_valence_admin::persistence::SubscriptionStore::list_by_topic(
                &valence,
                &topic_name,
            )
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list subscriptions: {e}")))?
            .len() as u32;

        let events = photon_valence_admin::persistence::EventStore::list_recent(&valence, 10000)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list events: {e}")))?;
        let now = chrono::Utc::now();
        let day_ago = now - chrono::Duration::hours(24);
        let stamps: Vec<_> = events
            .iter()
            .filter(|e| e.topic_name().as_str() == topic_name)
            .map(|e| *e.created_at())
            .collect();
        let event_count_24h = count_since(&stamps, day_ago);

        topics.push(TopicSummary {
            topic_name,
            keyed_by: desc.keyed_by.map(|s| s.to_string()),
            schema_json: desc.schema_json.to_string(),
            event_count_24h,
            subscription_count,
        });
    }
    sort_topics_by_name(&mut topics);
    Ok(topics)
}

/// Get a single topic by name.
#[uf_product_macros::server]
pub async fn get_topic(
    /// Name of the topic to look up.
    topic_name: String,
) -> Result<Option<TopicSummary>, ServerFnError> {
    validate_topic_name(&topic_name).map_err(ServerFnError::new)?;
    let topics = get_topics().await?;
    Ok(find_topic_by_name(&topics, &topic_name).cloned())
}

/// Get all subscriptions.
#[uf_product_macros::server]
pub async fn get_subscriptions() -> Result<Vec<SubscriptionSummary>, ServerFnError> {
    let valence = system_valence().await?;

    let subs = photon_valence_admin::persistence::SubscriptionStore::list(&valence)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list subscriptions: {e}")))?;

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
        list.push(SubscriptionSummary {
            subscription_id: sub_id,
            subscription_name: sub_name,
            topic_name,
            enabled: *s.enabled(),
            mode: s.mode().to_string(),
            topic_key_filter,
            checkpoint_lag: stub_checkpoint_lag(),
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
    validate_subscription_id(&id).map_err(ServerFnError::new)?;
    let subs = get_subscriptions().await?;
    Ok(find_subscription_by_id(&subs, &id).cloned())
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
        validate_topic_name(topic).map_err(ServerFnError::new)?;
        photon_valence_admin::persistence::EventStore::list_by_topic(
            &valence,
            topic,
            None,
            None,
            limit as usize,
        )
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to list events: {e}")))?
    } else {
        photon_valence_admin::persistence::EventStore::list_recent(&valence, limit as usize)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to list recent events: {e}")))?
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
    validate_event_id(&id).map_err(ServerFnError::new)?;
    let photon = photon_from_context()?;
    let valence = system_valence().await?;

    let meta = photon_valence_admin::persistence::EventStore::get_by_id(&valence, &id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to load projection: {e}")))?;

    let Some(meta) = meta else {
        return Ok(None);
    };

    let transport = photon
        .get_event(&id)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get transport event: {e}")))?;

    match transport {
        Some(t) => Ok(Some(EventDetail {
            event_id: id,
            topic_name: meta.topic_name().to_string(),
            topic_key: meta.topic_key().map(|s| s.to_string()),
            seq: *meta.seq(),
            created_at: meta.created_at().to_rfc3339(),
            delivery_status: meta.delivery_status().to_string(),
            payload_json: t.payload_json,
            actor_json: t.actor_json,
            transport_expired: false,
        })),
        None => Ok(Some(event_detail_transport_expired(
            id,
            meta.topic_name().to_string(),
            meta.topic_key().map(|s| s.to_string()),
            *meta.seq(),
            meta.created_at().to_rfc3339(),
            meta.delivery_status().to_string(),
        ))),
    }
}
