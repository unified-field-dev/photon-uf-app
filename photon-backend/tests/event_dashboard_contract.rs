//! Integration contracts for event/dashboard helpers backing
//! `get_dashboard_stats` / `get_recent_events` / `get_events` / `get_event`.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use chrono::{DateTime, Utc};
use photon_backend::{
    count_since, dashboard_stats, event_detail_transport_expired, event_summary_from_meta,
    format_delivery_preview, validate_event_id,
};

#[test]
fn dashboard_stats_aggregates_counts_happy_path() {
    let stats = dashboard_stats(2, 4, 9);
    assert_eq!(stats.topic_count, 2);
    assert_eq!(stats.subscription_count, 4);
    assert_eq!(stats.event_count_24h, 9);
}

#[test]
fn count_since_24h_window_happy_path() {
    let since = DateTime::parse_from_rfc3339("2026-07-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let inside = since + chrono::Duration::hours(1);
    let outside = since - chrono::Duration::hours(1);
    assert_eq!(count_since(&[outside, inside, since], since), 2);
}

#[test]
fn count_since_all_older_is_zero_sad() {
    let since = DateTime::parse_from_rfc3339("2026-07-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let older = since - chrono::Duration::hours(5);
    assert_eq!(count_since(&[older], since), 0);
}

#[test]
fn event_summary_list_row_preview_happy_path() {
    let row = event_summary_from_meta(
        "evt-1",
        "orders",
        Some("k1".into()),
        42,
        "2026-07-01T12:00:00Z",
        "delivered",
    );
    assert_eq!(row.event_id, "evt-1");
    assert_eq!(row.topic_name, "orders");
    assert_eq!(row.topic_key.as_deref(), Some("k1"));
    assert_eq!(row.seq, 42);
    assert_eq!(row.payload_preview, format_delivery_preview("delivered"));
}

#[test]
fn event_detail_transport_expired_shape_happy_path() {
    let detail = event_detail_transport_expired(
        "evt-gone",
        "orders",
        None,
        7,
        "2026-07-01T12:00:00Z",
        "delivered",
    );
    assert!(detail.transport_expired);
    assert!(detail.payload_json.is_null());
    assert!(detail.actor_json.is_null());
    assert_eq!(detail.event_id, "evt-gone");
}

#[test]
fn validate_event_id_accepts_id_happy_path() {
    validate_event_id("evt-123").expect("non-empty event id");
}

#[test]
fn validate_event_id_rejects_blank_sad() {
    let err = validate_event_id("").expect_err("blank");
    assert!(err.contains("required"), "{err}");
    let err = validate_event_id(" \t ").expect_err("whitespace");
    assert!(err.contains("required"), "{err}");
}
