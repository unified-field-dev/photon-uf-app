//! Pure topic / subscription / event / dashboard mapping helpers.

use chrono::{DateTime, Utc};

use crate::types::{DashboardStats, EventDetail, EventSummary, SubscriptionSummary, TopicSummary};

/// Formats the short payload-preview chip shown in event list rows.
#[must_use]
pub fn format_delivery_preview(delivery_status: &str) -> String {
    format!("[{delivery_status}]")
}

/// Locates a topic summary by exact name (used by `get_topic` detail lookups).
#[must_use]
pub fn find_topic_by_name<'a>(
    topics: &'a [TopicSummary],
    topic_name: &str,
) -> Option<&'a TopicSummary> {
    topics.iter().find(|t| t.topic_name == topic_name)
}

/// Locates a subscription summary by exact id (used by `get_subscription`).
#[must_use]
pub fn find_subscription_by_id<'a>(
    subs: &'a [SubscriptionSummary],
    id: &str,
) -> Option<&'a SubscriptionSummary> {
    subs.iter().find(|s| s.subscription_id == id)
}

/// Filters subscriptions attached to a topic (topic-detail page contract).
#[must_use]
pub fn filter_subscriptions_by_topic<'a>(
    subs: &'a [SubscriptionSummary],
    topic_name: &str,
) -> Vec<&'a SubscriptionSummary> {
    subs.iter().filter(|s| s.topic_name == topic_name).collect()
}

/// Counts timestamps at or after `since` (dashboard / topic 24h windows).
#[must_use]
pub fn count_since(timestamps: &[DateTime<Utc>], since: DateTime<Utc>) -> u64 {
    timestamps.iter().filter(|ts| **ts >= since).count() as u64
}

/// Builds dashboard KPI counters after registry and store queries resolve.
#[must_use]
pub const fn dashboard_stats(
    topic_count: u32,
    subscription_count: u32,
    event_count_24h: u64,
) -> DashboardStats {
    DashboardStats {
        topic_count,
        subscription_count,
        event_count_24h,
    }
}

/// Sorts topic summaries by name (stable list contract for `get_topics`).
pub fn sort_topics_by_name(topics: &mut [TopicSummary]) {
    topics.sort_by(|a, b| a.topic_name.cmp(&b.topic_name));
}

/// Builds an [`EventSummary`] list-row preview from projected metadata fields.
#[must_use]
pub fn event_summary_from_meta(
    event_id: impl Into<String>,
    topic_name: impl Into<String>,
    topic_key: Option<String>,
    seq: i64,
    created_at: impl Into<String>,
    delivery_status: &str,
) -> EventSummary {
    EventSummary {
        event_id: event_id.into(),
        topic_name: topic_name.into(),
        topic_key,
        seq,
        created_at: created_at.into(),
        payload_preview: format_delivery_preview(delivery_status),
    }
}

/// Builds an [`EventSummary`] from a transport-log event (ops browse via Photon list APIs).
#[must_use]
pub fn event_summary_from_transport(
    event_id: impl Into<String>,
    topic_name: impl Into<String>,
    topic_key: Option<String>,
    seq: i64,
    created_at: impl Into<String>,
) -> EventSummary {
    event_summary_from_meta(event_id, topic_name, topic_key, seq, created_at, "stored")
}

/// Builds [`EventDetail`] when Valence metadata exists but transport payload is gone.
#[must_use]
pub fn event_detail_transport_expired(
    event_id: impl Into<String>,
    topic_name: impl Into<String>,
    topic_key: Option<String>,
    seq: i64,
    created_at: impl Into<String>,
    delivery_status: impl Into<String>,
) -> EventDetail {
    EventDetail {
        event_id: event_id.into(),
        topic_name: topic_name.into(),
        topic_key,
        seq,
        created_at: created_at.into(),
        delivery_status: delivery_status.into(),
        payload_json: serde_json::Value::Null,
        actor_json: serde_json::Value::Null,
        transport_expired: true,
    }
}

/// Builds [`EventDetail`] from a live transport event.
#[must_use]
pub fn event_detail_from_transport(
    event_id: impl Into<String>,
    topic_name: impl Into<String>,
    topic_key: Option<String>,
    seq: i64,
    created_at: impl Into<String>,
    payload_json: serde_json::Value,
    actor_json: serde_json::Value,
) -> EventDetail {
    EventDetail {
        event_id: event_id.into(),
        topic_name: topic_name.into(),
        topic_key,
        seq,
        created_at: created_at.into(),
        delivery_status: "stored".into(),
        payload_json,
        actor_json,
        transport_expired: false,
    }
}

/// Builds a [`SubscriptionSummary`] from Photon `admin_snapshot` handler + checkpoint fields.
#[must_use]
pub fn subscription_summary_from_handler(
    registry_key: impl Into<String>,
    subscription_name: Option<String>,
    topic_name: impl Into<String>,
    mode: impl Into<String>,
    last_seq: Option<i64>,
    head_seq: Option<i64>,
) -> SubscriptionSummary {
    let topic_name = topic_name.into();
    let registry_key = registry_key.into();
    let display_name = subscription_name.unwrap_or_else(|| registry_key.clone());
    SubscriptionSummary {
        subscription_id: registry_key,
        subscription_name: display_name,
        topic_name,
        enabled: true,
        mode: mode.into(),
        topic_key_filter: None,
        checkpoint_lag: compute_checkpoint_lag(head_seq, last_seq),
        last_seq,
        last_processed_at: None,
    }
}

/// A checkpoint row: `(subscription_name, topic_name, topic_key, last_seq, head_seq)`.
pub type CheckpointRow = (String, String, Option<String>, Option<i64>, Option<i64>);

/// Matches a checkpoint `last_seq` for a handler subscription/topic pair.
#[must_use]
pub fn find_checkpoint_seq(
    checkpoints: &[CheckpointRow],
    subscription_name: Option<&str>,
    topic_name: &str,
) -> Option<i64> {
    let sub = subscription_name?;
    checkpoints
        .iter()
        .find(|(s, t, _, _, _)| s == sub && t == topic_name)
        .and_then(|(_, _, _, last_seq, _)| *last_seq)
}

/// Matches a checkpoint `head_seq` for a handler subscription/topic pair.
#[must_use]
pub fn find_checkpoint_head_seq(
    checkpoints: &[CheckpointRow],
    subscription_name: Option<&str>,
    topic_name: &str,
) -> Option<i64> {
    let sub = subscription_name?;
    checkpoints
        .iter()
        .find(|(s, t, _, _, _)| s == sub && t == topic_name)
        .and_then(|(_, _, _, _, head_seq)| *head_seq)
}

/// Builds a [`TopicSummary`] from registry metadata plus traffic counts.
#[must_use]
pub fn topic_summary(
    topic_name: impl Into<String>,
    keyed_by: Option<String>,
    schema_json: impl Into<String>,
    event_count_24h: u64,
    subscription_count: u32,
) -> TopicSummary {
    TopicSummary {
        topic_name: topic_name.into(),
        keyed_by,
        schema_json: schema_json.into(),
        event_count_24h,
        subscription_count,
    }
}

/// Lag between a topic's head sequence and a subscription's committed checkpoint.
///
/// A missing checkpoint is treated as "nothing processed yet" (full lag). A missing or
/// unsupported head seq is treated as zero head (no visible lag) rather than as an error.
#[must_use]
pub fn compute_checkpoint_lag(head_seq: Option<i64>, last_seq: Option<i64>) -> i64 {
    let head = head_seq.unwrap_or(0);
    let last = last_seq.unwrap_or(0);
    (head - last).max(0)
}
