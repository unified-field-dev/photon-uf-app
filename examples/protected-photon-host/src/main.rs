//! Protected `/photon` host: session auth gate + in-memory dashboard happy path.
//!
//! Copy surfaces for product hosts: this package's `Cargo.toml` + `main.rs`,
//! plus the product-mount dependency / Leptos sketches in the host README.
//! Oneshot path `/photon` matches Orbital app id/path `photon` / `/photon`
//! (see JSON `inventory`).
//!
//! Mirrors what a real host does before mounting [`photon_app::PhotonRoutes`]:
//! deny anonymous traffic under `/photon`, then serve the dashboard KPI shape
//! the UI's `get_dashboard_stats` server fn builds via `photon-backend`.
//!
//! ## When to use
//! Smoke the `/photon` auth + dashboard contract without a full Leptos SSR graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-photon-uf-app
//! cargo run -p protected-photon-host
//! ```
//!
//! ## Success
//! Stdout prints `protected_photon_host: OK — /photon deny/allow + dashboard KPIs`.
//!
//! ## Look next
//! Mount `<PhotonRoutes />` in a product host; wire Photon runtime + valence-admin.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

async fn photon_dashboard(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    let stats = photon_backend::dashboard_stats(4, 2, 17);
    Json(serde_json::json!({
        "path": "/photon",
        "user": session.user_id,
        "stats": stats,
        "inventory": {
            "app_id": "photon",
            "route_path": "/photon",
            "auth_gate": "RequireAuthenticated",
            "admin_permission": "PhotonAdmin",
        },
    }))
}

fn app() -> Router {
    Router::new()
        .route("/photon", get(photon_dashboard))
        .route_layer(from_fn(require_session))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    let denied = status_for("/photon", None).await;
    assert_eq!(denied, StatusCode::UNAUTHORIZED);

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/photon")
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], "/photon");
    assert_eq!(body["user"], "demo-ops");
    assert_eq!(body["stats"]["topic_count"], 4);
    assert_eq!(body["stats"]["subscription_count"], 2);
    assert_eq!(body["stats"]["event_count_24h"], 17);
    assert_eq!(body["inventory"]["app_id"], "photon");
    assert_eq!(body["inventory"]["route_path"], "/photon");
    assert_eq!(body["inventory"]["auth_gate"], "RequireAuthenticated");
    assert_eq!(body["inventory"]["admin_permission"], "PhotonAdmin");

    println!("protected_photon_host: OK — /photon deny/allow + dashboard KPIs");
}
