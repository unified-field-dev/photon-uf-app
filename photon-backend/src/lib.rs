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
//! | **Validate list/detail ids** | [`PhotonIdError`], [`validate_topic_name`], [`validate_subscription_id`], [`validate_event_id`] |
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
//! (`photon-app`); Photon transport, brokers, or `IsolatedLab` persistence (Photon core).
//!
//! ## Concern → API
//!
//! | Concern | API | Owner |
//! |---------|-----|-------|
//! | Id / name validation | [`PhotonIdError`], [`validate_topic_name`], [`validate_subscription_id`], [`validate_event_id`] | this crate |
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
//! | Detailed | `examples/protected-photon-host` (auth + dashboard KPIs; copy README) |

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
    clamp_event_list_limit, validate_event_id, validate_subscription_id, validate_topic_name,
    MAX_EVENT_LIST_LIMIT,
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
    fn validate_subscription_id_rejects_blank_sad() {
        assert_eq!(
            validate_subscription_id("").expect_err("blank"),
            PhotonIdError::EmptySubscriptionId
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
