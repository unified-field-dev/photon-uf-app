//! Pure backend contracts for the Photon UF app server surface.
//!
//! Leptos `#[server]` entrypoints in `photon-app` resolve Higgs / Photon request
//! context, then call these helpers so topic, subscription, event, and dashboard
//! shapes stay unit- and integration-testable without a full host or UI graph.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | **Validate list/detail ids** | [`validate_topic_name`], [`validate_subscription_id`], [`validate_event_id`] |
//! | **Topic list/detail mapping** | [`TopicSummary`], [`find_topic_by_name`], [`sort_topics_by_name`], [`topic_summary`] |
//! | **Subscription list/detail mapping** | [`SubscriptionSummary`], [`find_subscription_by_id`], [`filter_subscriptions_by_topic`], [`subscription_summary_from_handler`] |
//! | **Event list/detail + transport expiry** | [`EventSummary`], [`EventDetail`], [`event_summary_from_transport`], [`event_detail_from_transport`], [`event_detail_transport_expired`] |
//! | **Dashboard KPIs** | [`DashboardStats`], [`dashboard_stats`], [`count_since`] |
//! | **Event list limits** | [`clamp_event_list_limit`], [`MAX_EVENT_LIST_LIMIT`] |
//! | **UI pages / `#[server]` wrappers** | `photon-app` (not this crate) |
//!
//! ## Owns / does not own
//!
//! **Owns:** DTO shapes and pure mapping/validation helpers used by the Photon
//! ops UI server surface.
//!
//! **Does not own:** Leptos pages, Higgs `#[server]` wrappers, or route registration
//! (`photon-app`); Photon transport, brokers, or IsolatedLab persistence (Photon core).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Id / name validation | [`validate_topic_name`], [`validate_subscription_id`], [`validate_event_id`] | this crate |
//! | Topic summaries | [`TopicSummary`], [`find_topic_by_name`], [`sort_topics_by_name`] | this crate |
//! | Subscription summaries | [`SubscriptionSummary`], [`find_subscription_by_id`], [`filter_subscriptions_by_topic`] | this crate |
//! | Event summaries / detail | [`EventSummary`], [`EventDetail`], [`event_summary_from_transport`], [`event_detail_from_transport`] | this crate |
//! | Dashboard aggregates | [`DashboardStats`], [`dashboard_stats`], [`count_since`] | this crate |
//! | Pages, routes, server fns | `photon-app` (`PhotonRoutes`) | `photon-app` |
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Concern → API table above |
//! | Mid | This crate's unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-photon-host` |

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Summary of a Photon topic for list/detail views: registry metadata plus
/// recent traffic counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashboardStats {
    /// Total number of registered topics.
    pub topic_count: u32,
    /// Total number of registered subscriptions.
    pub subscription_count: u32,
    /// Number of events published across all topics in the last 24 hours.
    pub event_count_24h: u64,
}

/// Full detail record for a single event, including payload and actor context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    /// True when the transport record is no longer available for this event id.
    pub transport_expired: bool,
}

/// Rejects blank topic names before registry / detail lookups.
///
/// # Errors
///
/// Returns an error message when `topic_name` is empty or whitespace-only.
pub fn validate_topic_name(topic_name: &str) -> Result<(), String> {
    if topic_name.trim().is_empty() {
        Err("Photon topic name is required".to_string())
    } else {
        Ok(())
    }
}

/// Rejects blank subscription ids before detail lookups.
///
/// # Errors
///
/// Returns an error message when `id` is empty or whitespace-only.
pub fn validate_subscription_id(id: &str) -> Result<(), String> {
    if id.trim().is_empty() {
        Err("Photon subscription id is required".to_string())
    } else {
        Ok(())
    }
}

/// Rejects blank event ids before detail lookups.
///
/// # Errors
///
/// Returns an error message when `id` is empty or whitespace-only.
pub fn validate_event_id(id: &str) -> Result<(), String> {
    if id.trim().is_empty() {
        Err("Photon event id is required".to_string())
    } else {
        Ok(())
    }
}

/// Maximum events returned by a single `get_events` / `get_recent_events` call.
pub const MAX_EVENT_LIST_LIMIT: u32 = 100;

/// Cap requested event list limits to [`MAX_EVENT_LIST_LIMIT`].
#[must_use]
pub fn clamp_event_list_limit(limit: u32) -> usize {
    limit.min(MAX_EVENT_LIST_LIMIT) as usize
}

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

/// Builds a [`SubscriptionSummary`] from Photon admin_snapshot handler + checkpoint fields.
#[must_use]
pub fn subscription_summary_from_handler(
    registry_key: impl Into<String>,
    subscription_name: Option<String>,
    topic_name: impl Into<String>,
    mode: impl Into<String>,
    last_seq: Option<i64>,
) -> SubscriptionSummary {
    let topic_name = topic_name.into();
    let registry_key = registry_key.into();
    let display_name = subscription_name
        .clone()
        .unwrap_or_else(|| registry_key.clone());
    SubscriptionSummary {
        subscription_id: registry_key,
        subscription_name: display_name,
        topic_name,
        enabled: true,
        mode: mode.into(),
        topic_key_filter: None,
        checkpoint_lag: stub_checkpoint_lag(),
        last_seq,
        last_processed_at: None,
    }
}

/// Matches a checkpoint `last_seq` for a handler subscription/topic pair.
#[must_use]
pub fn find_checkpoint_seq(
    checkpoints: &[(String, String, Option<String>, Option<i64>)],
    subscription_name: Option<&str>,
    topic_name: &str,
) -> Option<i64> {
    let sub = subscription_name?;
    checkpoints
        .iter()
        .find(|(s, t, _, _)| s == sub && t == topic_name)
        .and_then(|(_, _, _, seq)| *seq)
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

/// Checkpoint lag placeholder until live lag is wired from Photon checkpoints vs head.
#[must_use]
pub const fn stub_checkpoint_lag() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_topic(name: &str) -> TopicSummary {
        TopicSummary {
            topic_name: name.into(),
            keyed_by: None,
            schema_json: "{}".into(),
            event_count_24h: 0,
            subscription_count: 0,
        }
    }

    fn sample_sub(id: &str, topic: &str) -> SubscriptionSummary {
        SubscriptionSummary {
            subscription_id: id.into(),
            subscription_name: "sub".into(),
            topic_name: topic.into(),
            enabled: true,
            mode: "at_least_once".into(),
            topic_key_filter: None,
            checkpoint_lag: stub_checkpoint_lag(),
            last_seq: None,
            last_processed_at: None,
        }
    }

    #[test]
    fn format_delivery_preview_wraps_status_happy_path() {
        assert_eq!(format_delivery_preview("delivered"), "[delivered]");
    }

    #[test]
    fn find_topic_by_name_resolves_exact_happy_path() {
        let topics = vec![sample_topic("alpha"), sample_topic("beta")];
        assert_eq!(
            find_topic_by_name(&topics, "beta").map(|t| t.topic_name.as_str()),
            Some("beta")
        );
    }

    #[test]
    fn find_topic_by_name_unknown_is_none_sad() {
        let topics = vec![sample_topic("alpha")];
        assert!(find_topic_by_name(&topics, "missing").is_none());
    }

    #[test]
    fn find_subscription_by_id_resolves_exact_happy_path() {
        let subs = vec![sample_sub("s1", "t"), sample_sub("s2", "t")];
        assert_eq!(
            find_subscription_by_id(&subs, "s1").map(|s| s.subscription_id.as_str()),
            Some("s1")
        );
    }

    #[test]
    fn find_subscription_by_id_unknown_is_none_sad() {
        let subs = vec![sample_sub("s1", "t")];
        assert!(find_subscription_by_id(&subs, "nope").is_none());
    }

    #[test]
    fn filter_subscriptions_by_topic_happy_path() {
        let subs = vec![
            sample_sub("a", "orders"),
            sample_sub("b", "payments"),
            sample_sub("c", "orders"),
        ];
        let filtered = filter_subscriptions_by_topic(&subs, "orders");
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|s| s.topic_name == "orders"));
    }

    #[test]
    fn filter_subscriptions_by_topic_unknown_empty_sad() {
        let subs = vec![sample_sub("a", "orders")];
        assert!(filter_subscriptions_by_topic(&subs, "missing").is_empty());
    }

    #[test]
    fn count_since_includes_boundary_excludes_older_happy_path() {
        let since = DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let older = since - chrono::Duration::hours(1);
        let on_boundary = since;
        let newer = since + chrono::Duration::hours(1);
        assert_eq!(count_since(&[older, on_boundary, newer], since), 2);
    }

    #[test]
    fn validate_topic_name_accepts_non_empty_happy_path() {
        validate_topic_name("orders").expect("non-empty");
        validate_topic_name("  payments  ").expect("trimmed non-empty");
    }

    #[test]
    fn validate_topic_name_rejects_blank_sad() {
        let err = validate_topic_name("").expect_err("blank");
        assert!(err.contains("required"), "{err}");
        let err = validate_topic_name("   ").expect_err("whitespace");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn validate_subscription_id_rejects_blank_sad() {
        let err = validate_subscription_id("").expect_err("blank");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn validate_event_id_rejects_blank_sad() {
        let err = validate_event_id("").expect_err("blank");
        assert!(err.contains("required"), "{err}");
    }

    #[test]
    fn dashboard_stats_shape_happy_path() {
        let stats = dashboard_stats(3, 5, 12);
        assert_eq!(stats.topic_count, 3);
        assert_eq!(stats.subscription_count, 5);
        assert_eq!(stats.event_count_24h, 12);
    }

    #[test]
    fn sort_topics_by_name_orders_lexicographically_happy_path() {
        let mut topics = vec![sample_topic("zeta"), sample_topic("alpha")];
        sort_topics_by_name(&mut topics);
        assert_eq!(topics[0].topic_name, "alpha");
        assert_eq!(topics[1].topic_name, "zeta");
    }

    #[test]
    fn event_summary_from_meta_preview_happy_path() {
        let row = event_summary_from_meta("e1", "t", None, 1, "2026-01-01T00:00:00Z", "delivered");
        assert_eq!(row.payload_preview, "[delivered]");
        assert_eq!(row.event_id, "e1");
    }

    #[test]
    fn event_detail_transport_expired_marks_null_payload_happy_path() {
        let detail =
            event_detail_transport_expired("e1", "t", None, 1, "2026-01-01T00:00:00Z", "delivered");
        assert!(detail.transport_expired);
        assert!(detail.payload_json.is_null());
        assert!(detail.actor_json.is_null());
    }

    #[test]
    fn topic_summary_serde_roundtrip_happy_path() {
        let topic = sample_topic("orders");
        let json = serde_json::to_string(&topic).expect("serialize");
        let back: TopicSummary = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, topic);
    }

    #[test]
    fn stub_checkpoint_lag_is_zero_happy_path() {
        assert_eq!(stub_checkpoint_lag(), 0);
    }

    #[test]
    fn subscription_summary_from_handler_uses_registry_key_happy_path() {
        let sub = subscription_summary_from_handler(
            "reg.key",
            Some("orders.sub".into()),
            "orders",
            "durable",
            Some(42),
        );
        assert_eq!(sub.subscription_id, "reg.key");
        assert_eq!(sub.subscription_name, "orders.sub");
        assert_eq!(sub.last_seq, Some(42));
        assert!(sub.enabled);
    }

    #[test]
    fn find_checkpoint_seq_matches_subscription_topic_happy_path() {
        let cps = vec![
            ("sub-a".into(), "orders".into(), None, Some(9)),
            ("sub-b".into(), "orders".into(), None, Some(3)),
        ];
        assert_eq!(find_checkpoint_seq(&cps, Some("sub-a"), "orders"), Some(9));
        assert_eq!(find_checkpoint_seq(&cps, None, "orders"), None);
        assert_eq!(find_checkpoint_seq(&cps, Some("missing"), "orders"), None);
    }

    #[test]
    fn event_summary_from_transport_marks_stored_happy_path() {
        let row = event_summary_from_transport("e1", "t", None, 1, "2026-01-01T00:00:00Z");
        assert_eq!(row.payload_preview, "[stored]");
    }

    #[test]
    fn clamp_event_list_limit_caps_oversized_sad() {
        assert_eq!(
            clamp_event_list_limit(10_000),
            MAX_EVENT_LIST_LIMIT as usize
        );
        assert_eq!(
            clamp_event_list_limit(MAX_EVENT_LIST_LIMIT),
            MAX_EVENT_LIST_LIMIT as usize
        );
    }

    #[test]
    fn clamp_event_list_limit_preserves_small_happy_path() {
        assert_eq!(clamp_event_list_limit(20), 20);
        assert_eq!(clamp_event_list_limit(0), 0);
    }
}
