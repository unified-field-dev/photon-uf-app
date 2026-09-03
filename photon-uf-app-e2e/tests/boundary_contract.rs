//! Valence + Photon ops boundary contracts for the lab host.
//!
//! These are not Playwright; they assert durable topic/subscription/event
//! postconditions on the in-process mem Photon after `init_e2e_valence`.

#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use photon_backend::ops::{
    list_subscriptions, load_dashboard_stats, load_event, load_events, load_recent_events,
    load_subscription, load_topic, load_topics, require_photon, require_photon_admin,
    require_session_user, OpsError,
};
use photon_uf_app_e2e::{e2e_fixtures, e2e_photon, init_e2e_valence};

#[tokio::test]
async fn ops_auth_guards_sad() {
    assert_eq!(
        require_session_user(None).unwrap_err(),
        OpsError::AuthenticationRequired
    );
    assert!(require_session_user(Some("user:admin")).is_ok());

    assert_eq!(
        require_photon_admin(false).unwrap_err(),
        OpsError::PermissionDenied
    );
    assert!(require_photon_admin(true).is_ok());

    assert_eq!(
        require_photon(None).err(),
        Some(OpsError::PhotonNotInContext)
    );
}

#[tokio::test]
async fn ops_dashboard_seeded_happy() {
    init_e2e_valence().await;
    let photon = e2e_photon();
    let stats = load_dashboard_stats(&photon).await.expect("dashboard");
    assert!(stats.topic_count >= 1, "registry must include seeded topic");
    assert!(
        stats.subscription_count >= 1,
        "admin_snapshot must list durable handler"
    );
    assert!(
        stats.event_count_24h >= 1,
        "published event must count in 24h window"
    );
}

#[tokio::test]
async fn ops_topics_list_and_unknown_sad() {
    init_e2e_valence().await;
    let photon = e2e_photon();
    let fixtures = e2e_fixtures();
    let topics = load_topics(&photon).await.expect("topics");
    assert!(
        topics.iter().any(|t| t.topic_name == fixtures.topic_name),
        "seeded topic missing: {topics:?}"
    );
    let detail = load_topic(&photon, &fixtures.topic_name)
        .await
        .expect("topic detail");
    assert_eq!(
        detail.map(|t| t.topic_name).as_deref(),
        Some(fixtures.topic_name.as_str())
    );

    let missing = load_topic(&photon, "test.photon.e2e.missing")
        .await
        .expect("unknown topic");
    assert!(missing.is_none());

    let blank = load_topic(&photon, "").await.expect_err("blank topic");
    assert!(matches!(blank, OpsError::InvalidId(_)));
}

#[tokio::test]
async fn ops_subs_list_and_unknown_sad() {
    init_e2e_valence().await;
    let photon = e2e_photon();
    let fixtures = e2e_fixtures();
    let subs = list_subscriptions(&photon).await.expect("subs");
    assert!(
        subs.iter()
            .any(|s| s.subscription_id == fixtures.subscription_id),
        "handler missing: {subs:?}"
    );
    let detail = load_subscription(&photon, &fixtures.subscription_id)
        .await
        .expect("sub detail");
    assert_eq!(
        detail.map(|s| s.subscription_id).as_deref(),
        Some(fixtures.subscription_id.as_str())
    );

    let missing = load_subscription(&photon, "no-such-handler")
        .await
        .expect("unknown sub");
    assert!(missing.is_none());
}

#[tokio::test]
async fn ops_events_list_detail_and_unknown_sad() {
    init_e2e_valence().await;
    let photon = e2e_photon();
    let fixtures = e2e_fixtures();
    let recent = load_recent_events(&photon, 50).await.expect("recent");
    assert!(!recent.is_empty());
    assert!(
        recent.iter().any(|e| e.event_id == fixtures.event_id),
        "seeded event missing: {recent:?}"
    );

    let by_topic = load_events(&photon, Some(&fixtures.topic_name), 50)
        .await
        .expect("by topic");
    assert!(by_topic.iter().any(|e| e.event_id == fixtures.event_id));

    let detail = load_event(&photon, &fixtures.event_id)
        .await
        .expect("detail");
    let detail = detail.expect("event exists");
    assert_eq!(detail.event_id, fixtures.event_id);
    assert_eq!(detail.topic_name, fixtures.topic_name);

    let missing = load_event(&photon, "no-such-event-id")
        .await
        .expect("unknown event");
    assert!(missing.is_none());

    let blank = load_event(&photon, "").await.expect_err("blank event id");
    assert!(matches!(blank, OpsError::InvalidId(_)));
}

#[tokio::test]
async fn require_photon_with_e2e_handle_happy() {
    init_e2e_valence().await;
    let photon = require_photon(Some(e2e_photon())).expect("photon in context");
    assert!(load_topics(&photon)
        .await
        .expect("topics")
        .iter()
        .any(|t| t.topic_name == e2e_fixtures().topic_name));
}
