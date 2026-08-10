//! Product surface contracts for photon-app (sibling crate).
//!
//! Lives under `photon-backend` so CI can gate route/testid/auth/admin needles
//! without compiling Orbital/turf UI when host pins churn. Pattern matches
//! gauge `gauge/tests/product_surface.rs` and lepton-uf-app
//! `lepton-shell/tests/product_surface.rs`.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read_app(rel: &str) -> String {
    let path = workspace_root().join("photon-app").join("src").join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn photon_routes_mount_happy_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("photon")"#,
        r#"path!("")"#,
        r#"path!("topics")"#,
        r#"path!("topics/:topic_name")"#,
        r#"path!("subscriptions")"#,
        r#"path!("subscriptions/:id")"#,
        r#"path!("events")"#,
        r#"path!("events/:id")"#,
        "PhotonLayoutRouteView",
        "id: \"photon\"",
        "route_path: \"/photon\"",
        "permission_manifest: permissions::PhotonPermission",
    ] {
        assert!(
            lib.contains(needle),
            "PhotonRoutes / uf_app missing `{needle}`"
        );
    }
}

#[test]
fn photon_routes_drop_leaf_sad_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("topics/:topic_name")"#,
        r#"path!("subscriptions/:id")"#,
        r#"path!("events/:id")"#,
    ] {
        assert!(
            lib.contains(needle),
            "removing `{needle}` drops a Photon ops funnel entry"
        );
    }
    assert!(
        !lib.contains("unimplemented!"),
        "PhotonRoutes must not ship unimplemented placeholders"
    );
}

#[test]
fn uf_app_wrong_id_sad_path() {
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"photon\""),
        "wrong uf_app id breaks Orbital host registration"
    );
    assert!(
        !lib.contains("id: \"photon-app\""),
        "uf_app id must stay `photon` (product route id), not crate name photon-app"
    );
}

#[test]
fn layout_auth_gate_and_nav_happy_path() {
    let layout = read_app("layout.rs");
    for needle in [
        "photon-app-root",
        "RequireAuthenticated",
        "Outlet",
        "nav-photon-dashboard",
        "nav-photon-topics",
        "nav-photon-subscriptions",
        "nav-photon-events",
        "AppBarUserMenu",
        "UnifiedFieldShellLayout",
    ] {
        assert!(
            layout.contains(needle),
            "PhotonLayout missing contract `{needle}`"
        );
    }
}

#[test]
fn layout_drop_auth_guard_sad_path() {
    let layout = read_app("layout.rs");
    assert!(
        layout.contains("RequireAuthenticated") && layout.contains("<Outlet />"),
        "removing RequireAuthenticated opens /photon pages to anonymous sessions"
    );
}

#[test]
fn layout_missing_nav_sad_path() {
    let layout = read_app("layout.rs");
    for id in [
        "nav-photon-dashboard",
        "nav-photon-topics",
        "nav-photon-subscriptions",
        "nav-photon-events",
    ] {
        assert!(
            layout.contains(id),
            "dropping `{id}` breaks operator left-nav contract"
        );
    }
}

#[test]
fn ops_reads_require_photon_admin_happy_path() {
    let server = read_app("server.rs");
    for fn_name in [
        "get_dashboard_stats",
        "get_recent_events",
        "get_topics",
        "get_topic",
        "get_subscriptions",
        "get_subscription",
        "get_events",
        "get_event",
    ] {
        assert!(server.contains(fn_name), "server missing `{fn_name}`");
    }
    let admin_attr = r#"permission = "PhotonAdmin""#;
    assert!(
        server.matches(admin_attr).count() >= 8,
        "ops read server fns must carry PhotonAdmin permission attribute"
    );
    assert!(
        server.contains("PHOTON_ADMIN_PERMISSION: &str = \"PhotonAdmin\""),
        "PHOTON_ADMIN_PERMISSION constant must stay PhotonAdmin"
    );
}

#[test]
fn ops_reads_drop_photon_admin_sad_path() {
    let server = read_app("server.rs");
    let admin_attr = r#"permission = "PhotonAdmin""#;
    assert!(
        server.matches(admin_attr).count() >= 8,
        "dropping PhotonAdmin from any get_* opens ops reads without admin gate"
    );
    assert!(
        !server.contains(r#"permission = "GaugeAdmin""#),
        "Photon ops must not gate on GaugeAdmin"
    );
}

#[test]
fn server_require_session_happy_path() {
    let server = read_app("server.rs");
    assert!(
        server.contains("fn require_session")
            && server.contains("Authentication required")
            && server.contains("session_user_id()"),
        "server must fail closed without a session"
    );
    assert!(
        server.contains("Photon not in request context"),
        "missing Photon context must surface a typed ServerFnError message"
    );
    for call_site in ["get_dashboard_stats", "get_topics", "get_event"] {
        assert!(server.contains(call_site), "server missing `{call_site}`");
    }
}

#[test]
fn server_drop_require_session_on_get_topics_sad_path() {
    let server = read_app("server.rs");
    let start = server.find("pub async fn get_topics").expect("get_topics");
    let body = &server[start..start + 450.min(server.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "get_topics must call require_session before Photon IO"
    );
}

#[test]
fn index_pages_testid_and_list_bindings_happy_path() {
    let dashboard = read_app("pages/dashboard.rs");
    for needle in [
        "photon-dashboard",
        "get_dashboard_stats",
        "get_recent_events",
        "get_subscriptions",
    ] {
        assert!(
            dashboard.contains(needle),
            "PhotonDashboardPage missing `{needle}`"
        );
    }

    let topics = read_app("pages/topics.rs");
    for needle in ["photon-topics", "get_topics"] {
        assert!(
            topics.contains(needle),
            "PhotonTopicsIndexPage missing `{needle}`"
        );
    }

    let subscriptions = read_app("pages/subscriptions.rs");
    for needle in ["photon-subscriptions", "get_subscriptions"] {
        assert!(
            subscriptions.contains(needle),
            "PhotonSubscriptionsIndexPage missing `{needle}`"
        );
    }

    let events = read_app("pages/events.rs");
    for needle in ["photon-events", "get_events", "get_topics"] {
        assert!(
            events.contains(needle),
            "PhotonEventsIndexPage missing `{needle}`"
        );
    }
}

#[test]
fn index_drop_dashboard_testid_sad_path() {
    let dashboard = read_app("pages/dashboard.rs");
    assert!(
        dashboard.contains("data_testid=\"photon-dashboard\""),
        "dropping photon-dashboard breaks host / future Playwright parity"
    );
    let topics = read_app("pages/topics.rs");
    assert!(
        topics.contains("data_testid=\"photon-topics\""),
        "dropping photon-topics breaks host / future Playwright parity"
    );
    let subscriptions = read_app("pages/subscriptions.rs");
    assert!(
        subscriptions.contains("data_testid=\"photon-subscriptions\""),
        "dropping photon-subscriptions breaks host / future Playwright parity"
    );
    let events = read_app("pages/events.rs");
    assert!(
        events.contains("data_testid=\"photon-events\""),
        "dropping photon-events breaks host / future Playwright parity"
    );
}

#[test]
fn detail_pages_testid_and_bindings_happy_path() {
    let topic = read_app("pages/topic_detail.rs");
    for needle in [
        "photon-topic-detail",
        "get_topic",
        "get_events",
        "get_subscriptions",
    ] {
        assert!(
            topic.contains(needle),
            "PhotonTopicDetailPage missing `{needle}`"
        );
    }

    let subscription = read_app("pages/subscription_detail.rs");
    for needle in [
        "photon-subscription-detail",
        "get_subscription",
        "get_events",
    ] {
        assert!(
            subscription.contains(needle),
            "PhotonSubscriptionDetailPage missing `{needle}`"
        );
    }

    let event = read_app("pages/event_detail.rs");
    for needle in ["photon-event-detail", "get_event"] {
        assert!(
            event.contains(needle),
            "PhotonEventDetailPage missing `{needle}`"
        );
    }
}

#[test]
fn detail_pages_missing_bindings_sad_path() {
    let topic = read_app("pages/topic_detail.rs");
    assert!(
        topic.contains("get_topic"),
        "topic detail must bind get_topic"
    );
    let subscription = read_app("pages/subscription_detail.rs");
    assert!(
        subscription.contains("get_subscription"),
        "subscription detail must bind get_subscription"
    );
    let event = read_app("pages/event_detail.rs");
    assert!(
        event.contains("get_event"),
        "event detail must bind get_event"
    );
    assert!(
        !topic.contains("unimplemented!")
            && !subscription.contains("unimplemented!")
            && !event.contains("unimplemented!"),
        "detail pages must not ship unimplemented placeholders"
    );
}

#[test]
fn permission_manifest_photon_admin_happy_path() {
    let perms = read_app("permissions.rs");
    for needle in [
        "domain_key = \"photon\"",
        "PhotonAdmin",
        "UfPermissionManifest",
    ] {
        assert!(
            perms.contains(needle),
            "PhotonPermission manifest missing `{needle}`"
        );
    }
}

#[test]
fn protected_photon_host_matches_uf_app_happy_path() {
    let host =
        fs::read_to_string(workspace_root().join("examples/protected-photon-host/src/main.rs"))
            .expect("protected-photon-host main.rs");
    for needle in [
        "\"app_id\": \"photon\"",
        "\"route_path\": \"/photon\"",
        "\"auth_gate\": \"RequireAuthenticated\"",
        "\"admin_permission\": \"PhotonAdmin\"",
        "dashboard_stats",
    ] {
        assert!(
            host.contains(needle),
            "protected-photon-host missing contract `{needle}`"
        );
    }
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"photon\"") && lib.contains("route_path: \"/photon\""),
        "host inventory must stay aligned with uf_app!"
    );
    let layout = read_app("layout.rs");
    assert!(
        layout.contains("RequireAuthenticated"),
        "host auth_gate must stay aligned with PhotonLayout guard"
    );
    let perms = read_app("permissions.rs");
    assert!(
        perms.contains("PhotonAdmin"),
        "host admin_permission must stay aligned with PhotonPermission"
    );
}

#[test]
fn lazy_routes_wire_pages_happy_path() {
    let lazy = read_app("lazy_routes.rs");
    for needle in [
        "PhotonDashboardPage",
        "PhotonTopicsIndexPage",
        "PhotonTopicDetailPage",
        "PhotonSubscriptionsIndexPage",
        "PhotonSubscriptionDetailPage",
        "PhotonEventsIndexPage",
        "PhotonEventDetailPage",
        "PhotonLayout",
    ] {
        assert!(
            lazy.contains(needle),
            "lazy_routes missing page wire `{needle}`"
        );
    }
}

#[test]
fn ops_path_helpers_encode_segments_happy_path() {
    let events_table = read_app("components/events_table.rs");
    let topic_card = read_app("components/topic_card.rs");
    let sub_card = read_app("components/subscription_card.rs");
    let topic_subs = read_app("components/topic_subscriptions_table.rs");
    let active_subs = read_app("components/active_subscriptions_table.rs");
    for (label, src) in [
        ("events_table", events_table.as_str()),
        ("topic_card", topic_card.as_str()),
        ("subscription_card", sub_card.as_str()),
        ("topic_subscriptions_table", topic_subs.as_str()),
        ("active_subscriptions_table", active_subs.as_str()),
    ] {
        assert!(
            src.contains("photon_backend::photon_")
                || src.contains("photon_event_path")
                || src.contains("photon_topic_path")
                || src.contains("photon_subscription_path"),
            "{label} must build detail hrefs via photon_backend path helpers"
        );
        assert!(
            !src.contains("crate::paths::topic(")
                && !src.contains("crate::paths::event(")
                && !src.contains("crate::paths::subscription("),
            "{label} must not interpolate raw ids into orbital paths::*"
        );
    }
}

#[test]
fn ops_path_helpers_drop_encoding_sad_path() {
    let topic_card = read_app("components/topic_card.rs");
    assert!(
        topic_card.contains("photon_backend::photon_topic_path"),
        "dropping photon_topic_path reopens path-segment smuggling via topic names"
    );
    let events_table = read_app("components/events_table.rs");
    assert!(
        events_table.contains("photon_backend::photon_event_path"),
        "dropping photon_event_path reopens path-segment smuggling via event ids"
    );
}
