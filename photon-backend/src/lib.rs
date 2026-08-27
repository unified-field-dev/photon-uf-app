//! Pure backend contracts for the Photon ops UI server surface.
//!
//! DTO shapes and pure mapping/validation helpers that `photon-app` `#[server]` functions
//! call after resolving Higgs and Photon request context. Keeps topic, subscription, event,
//! and dashboard contracts unit-testable without a Leptos host or UI graph.
//!
//! ## Features
//!
//! - **Id validation** — Reject blank, oversized, or path-unsafe topic names, subscription
//!   ids, and event ids before registry or store lookups.
//!   [Get started](#validate-ids)
//! - **Topic/subscription/event mapping** — Pure helpers that build UI DTOs from registry
//!   rows, admin snapshot handlers, and transport events.
//!   [Get started](#map-topic-subscription-event)
//! - **Dashboard aggregates** — KPI counters and 24-hour event windows via [`dashboard_stats`]
//!   and [`count_since`]. [Get started](#dashboard-kpis)
//! - **Ops path encoding** — Percent-encode path segments for `/photon` hrefs via
//!   [`encode_ops_path_segment`], [`photon_topic_path`], [`photon_subscription_path`], and
//!   [`photon_event_path`].
//! - **Event list limits** — Cap `get_events` / `get_recent_events` list size with
//!   [`clamp_event_list_limit`] and [`MAX_EVENT_LIST_LIMIT`].
//!
//! ## Validate ids
//!
//! Ops UI detail lookups reject ids that would break routing or leak path segments into
//! Photon IO. [`validate_topic_name`], [`validate_subscription_id`], and [`validate_event_id`]
//! run before `photon-app` server functions call registry or store APIs — call them in
//! custom wrappers when you add new read paths that accept path or query parameters.
//!
//! **Prerequisites:** None beyond importing this crate; validators are synchronous and infallible
//! except for returning [`PhotonIdError`].
//!
//! ```rust,ignore
//! use photon_backend::{
//!     validate_topic_name, validate_subscription_id, validate_event_id, PhotonIdError,
//! };
//!
//! validate_topic_name("orders").expect("valid topic");
//! assert_eq!(
//!     validate_topic_name("").unwrap_err(),
//!     PhotonIdError::EmptyTopicName
//! );
//! validate_subscription_id("reg.key").expect("valid subscription");
//! validate_event_id("ev-1").expect("valid event");
//! ```
//!
//! On success validators return `Ok(())` and the trimmed id is safe for lookup. Blank,
//! oversized, control-character, slash, backslash, or `.` / `..` names map to typed
//! [`PhotonIdError`] variants with operator-facing messages.
//!
//! ## Map topic subscription event
//!
//! Mapping helpers turn Photon registry rows and transport events into serde-friendly
//! DTOs the UI can render without touching Photon internals. [`topic_summary`] and
//! [`find_topic_by_name`] back topic list/detail pages; [`subscription_summary_from_handler`]
//! and [`filter_subscriptions_by_topic`] shape subscription tables; [`event_summary_from_transport`]
//! builds list-row previews with `[stored]` / delivery-status chips.
//!
//! **Prerequisites:** Caller already loaded registry topics, admin snapshot handlers, or
//! transport events from Photon — these functions do not perform IO.
//!
//! ```rust,ignore
//! use photon_backend::{
//!     topic_summary, subscription_summary_from_handler, event_summary_from_transport,
//!     find_topic_by_name, TopicSummary,
//! };
//!
//! let topic = topic_summary("orders", None, "{}", 5, 2);
//! assert_eq!(topic.topic_name, "orders");
//!
//! let sub = subscription_summary_from_handler(
//!     "reg.key",
//!     Some("orders.sub".into()),
//!     "orders",
//!     "durable",
//!     Some(42),
//! );
//! assert_eq!(sub.subscription_id, "reg.key");
//!
//! let row = event_summary_from_transport("e1", "orders", None, 1, "2026-01-01T00:00:00Z");
//! assert_eq!(row.payload_preview, "[stored]");
//!
//! let topics = vec![topic];
//! assert_eq!(find_topic_by_name(&topics, "orders").map(|t| t.topic_name.as_str()), Some("orders"));
//! ```
//!
//! On success helpers return populated [`TopicSummary`], [`SubscriptionSummary`], or
//! [`EventSummary`] rows ready for JSON serialization. Lookup helpers return `None` when
//! the id or name is absent from the caller-provided slice.
//!
//! ## Dashboard KPIs
//!
//! Dashboard KPI aggregates provide registry size and recent event volume counters
//! without UI-specific formatting. [`dashboard_stats`] packages topic, subscription,
//! and 24-hour event counts into [`DashboardStats`]; [`count_since`] filters timestamp
//! slices for per-topic 24h windows after the caller loads event timestamps from Photon.
//!
//! **Prerequisites:** Caller supplies counts from registry and event store queries — these
//! helpers do not call Photon.
//!
//! ```rust,ignore
//! use photon_backend::{dashboard_stats, count_since, DashboardStats};
//! use chrono::{DateTime, Utc};
//!
//! let stats: DashboardStats = dashboard_stats(3, 5, 12);
//! assert_eq!(stats.topic_count, 3);
//! assert_eq!(stats.subscription_count, 5);
//! assert_eq!(stats.event_count_24h, 12);
//!
//! let since = DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
//!     .unwrap()
//!     .with_timezone(&Utc);
//! assert_eq!(count_since(&[since], since), 1);
//! ```
//!
//! On success `stats` carries the three KPI fields consumed by `photon-app` dashboard
//! server functions and `count_since` returns how many timestamps fall on or after the cutoff.
//!
//! ## Examples ladder
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | [Validate ids](#validate-ids) |
//! | Mid | This crate's unit + integ suites (`docs/VERIFICATION.md`) |
//! | Detailed | `examples/protected-photon-host` (auth + dashboard KPIs) |

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod mapping;
mod types;
mod validate;

pub use mapping::{
    count_since, dashboard_stats, event_detail_from_transport, event_detail_transport_expired,
    event_summary_from_meta, event_summary_from_transport, filter_subscriptions_by_topic,
    find_checkpoint_seq, find_subscription_by_id, find_topic_by_name, format_delivery_preview,
    sort_topics_by_name, stub_checkpoint_lag, subscription_summary_from_handler, topic_summary,
};
pub use types::{
    DashboardStats, EventDetail, EventSummary, PhotonIdError, SubscriptionSummary, TopicSummary,
};
pub use validate::{
    clamp_event_list_limit, encode_ops_path_segment, photon_event_path, photon_subscription_path,
    photon_topic_path, validate_event_id, validate_subscription_id, validate_topic_name,
    MAX_EVENT_LIST_LIMIT, MAX_PHOTON_ID_CHARS,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

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
        assert_eq!(filter_subscriptions_by_topic(&subs, "missing").len(), 0);
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
        validate_topic_name("orders.v1").expect("dot in name");
    }

    #[test]
    fn validate_topic_name_rejects_blank_sad() {
        assert_eq!(
            validate_topic_name("").expect_err("blank"),
            PhotonIdError::EmptyTopicName
        );
        assert_eq!(
            validate_topic_name("   ").expect_err("whitespace"),
            PhotonIdError::EmptyTopicName
        );
        assert!(PhotonIdError::EmptyTopicName
            .to_string()
            .contains("required"));
    }

    #[test]
    fn validate_topic_name_rejects_slash_control_dotdot_sad() {
        assert_eq!(
            validate_topic_name("a/b").expect_err("slash"),
            PhotonIdError::UnsafeTopicName
        );
        assert_eq!(
            validate_topic_name("a\\b").expect_err("backslash"),
            PhotonIdError::UnsafeTopicName
        );
        assert_eq!(
            validate_topic_name("a\tb").expect_err("control"),
            PhotonIdError::UnsafeTopicName
        );
        assert_eq!(
            validate_topic_name("..").expect_err("dotdot"),
            PhotonIdError::UnsafeTopicName
        );
        assert_eq!(
            validate_topic_name(".").expect_err("dot"),
            PhotonIdError::UnsafeTopicName
        );
    }

    #[test]
    fn validate_topic_name_rejects_oversized_sad() {
        let oversized: String = "t".repeat(MAX_PHOTON_ID_CHARS + 1);
        assert_eq!(
            validate_topic_name(&oversized).expect_err("too long"),
            PhotonIdError::TopicNameTooLong
        );
    }

    #[test]
    fn validate_subscription_id_rejects_blank_sad() {
        assert_eq!(
            validate_subscription_id("").expect_err("blank"),
            PhotonIdError::EmptySubscriptionId
        );
    }

    #[test]
    fn validate_subscription_id_rejects_slash_sad() {
        assert_eq!(
            validate_subscription_id("reg/key").expect_err("slash"),
            PhotonIdError::UnsafeSubscriptionId
        );
    }

    #[test]
    fn validate_event_id_rejects_blank_sad() {
        assert_eq!(
            validate_event_id("").expect_err("blank"),
            PhotonIdError::EmptyEventId
        );
    }

    #[test]
    fn validate_event_id_rejects_control_sad() {
        assert_eq!(
            validate_event_id("ev\nid").expect_err("control"),
            PhotonIdError::UnsafeEventId
        );
    }

    #[test]
    fn encode_ops_path_segment_encodes_slash_and_space_happy_path() {
        assert_eq!(encode_ops_path_segment("orders"), "orders");
        assert_eq!(encode_ops_path_segment("a/b"), "a%2Fb");
        assert_eq!(encode_ops_path_segment("a b"), "a%20b");
        assert_eq!(encode_ops_path_segment("a\\b"), "a%5Cb");
    }

    #[test]
    fn photon_ops_paths_encode_segments_happy_path() {
        assert_eq!(photon_topic_path("a/b"), "/photon/topics/a%2Fb");
        assert_eq!(
            photon_subscription_path("reg/key"),
            "/photon/subscriptions/reg%2Fkey"
        );
        assert_eq!(photon_event_path("e 1"), "/photon/events/e%201");
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
