//! Integration contracts for topic/subscription helpers backing
//! `get_topics` / `get_topic` / `get_subscriptions` / `get_subscription`.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use photon_backend::{
    filter_subscriptions_by_topic, find_subscription_by_id, find_topic_by_name,
    sort_topics_by_name, stub_checkpoint_lag, validate_subscription_id, validate_topic_name,
    SubscriptionSummary, TopicSummary,
};

fn sample_topic(name: &str, subs: u32) -> TopicSummary {
    TopicSummary {
        topic_name: name.into(),
        keyed_by: Some("order_id".into()),
        schema_json: r#"{"type":"object"}"#.into(),
        event_count_24h: 4,
        subscription_count: subs,
    }
}

fn sample_sub(id: &str, topic: &str, enabled: bool) -> SubscriptionSummary {
    SubscriptionSummary {
        subscription_id: id.into(),
        subscription_name: format!("sub-{id}"),
        topic_name: topic.into(),
        enabled,
        mode: "at_least_once".into(),
        topic_key_filter: None,
        checkpoint_lag: stub_checkpoint_lag(),
        last_seq: Some(10),
        last_processed_at: None,
    }
}

#[test]
fn get_topics_list_sorted_and_named_happy_path() {
    let mut topics = vec![
        sample_topic("zeta.events", 1),
        sample_topic("alpha.events", 2),
    ];
    sort_topics_by_name(&mut topics);
    assert_eq!(topics[0].topic_name, "alpha.events");
    assert_eq!(topics[1].topic_name, "zeta.events");
    for t in &topics {
        assert!(!t.topic_name.trim().is_empty());
        assert!(!t.schema_json.is_empty());
    }
}

#[test]
fn get_topic_detail_matches_list_entry_happy_path() {
    let topics = vec![sample_topic("orders", 3), sample_topic("payments", 1)];
    let detail = find_topic_by_name(&topics, "orders").expect("listed topic must resolve");
    assert_eq!(detail.topic_name, "orders");
    assert_eq!(detail.subscription_count, 3);
    assert_eq!(detail.keyed_by.as_deref(), Some("order_id"));
}

#[test]
fn get_topic_unknown_name_is_none_sad() {
    let topics = vec![sample_topic("orders", 1)];
    assert!(find_topic_by_name(&topics, "__photon_uf_app_no_such_topic__").is_none());
}

#[test]
fn get_subscription_detail_matches_list_entry_happy_path() {
    let subs = vec![
        sample_sub("sub-1", "orders", true),
        sample_sub("sub-2", "payments", false),
    ];
    let detail = find_subscription_by_id(&subs, "sub-2").expect("listed sub must resolve");
    assert_eq!(detail.subscription_id, "sub-2");
    assert_eq!(detail.topic_name, "payments");
    assert!(!detail.enabled);
    assert_eq!(detail.checkpoint_lag, 0);
}

#[test]
fn get_subscription_unknown_id_is_none_sad() {
    let subs = vec![sample_sub("sub-1", "orders", true)];
    assert!(find_subscription_by_id(&subs, "__photon_uf_app_no_such_sub__").is_none());
}

#[test]
fn topic_detail_filters_subscriptions_for_topic_happy_path() {
    let subs = vec![
        sample_sub("a", "orders", true),
        sample_sub("b", "payments", true),
        sample_sub("c", "orders", false),
    ];
    let filtered = filter_subscriptions_by_topic(&subs, "orders");
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|s| s.topic_name == "orders"));
}

#[test]
fn topic_detail_filters_subscriptions_unknown_topic_empty_sad() {
    let subs = vec![sample_sub("a", "orders", true)];
    assert!(filter_subscriptions_by_topic(&subs, "__missing_topic__").is_empty());
}

#[test]
fn validate_topic_name_accepts_table_happy_path() {
    validate_topic_name("ops.orders").expect("non-empty topic");
}

#[test]
fn validate_topic_name_rejects_blank_sad() {
    let err = validate_topic_name("").expect_err("blank name");
    assert!(err.contains("required"), "{err}");
    let err = validate_topic_name("   ").expect_err("whitespace");
    assert!(err.contains("required"), "{err}");
}

#[test]
fn validate_subscription_id_rejects_blank_sad() {
    let err = validate_subscription_id("").expect_err("blank id");
    assert!(err.contains("required"), "{err}");
}

#[test]
fn validate_subscription_id_accepts_id_happy_path() {
    validate_subscription_id("sub-abc").expect("non-empty id");
}
