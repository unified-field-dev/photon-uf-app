//! Harness-only seed endpoint for Playwright.

use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use uf_help::HelpVisitRecord;

use crate::e2e_valence::e2e_fixtures;
use crate::gate_demos::{write_e2e_auth_kind, E2eAuthKind};

#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// `anonymous` | `admin` | `outsider` | `unverified`
    #[serde(default = "default_auth")]
    pub auth: String,
    /// When true, leave Help tour progress unset so spotlight green paths can run.
    #[serde(default)]
    pub help_tour: bool,
}

fn default_auth() -> String {
    E2eAuthKind::Anonymous.as_str().to_string()
}

/// Serde-stable JSON for every Photon Help inventory step (replay=false).
fn photon_help_seen_json() -> String {
    photon_app::ensure_help_steps_linked();
    let visits: Vec<HelpVisitRecord> = uf_help::collect_help_steps()
        .into_iter()
        .filter(|s| s.route.starts_with("/photon"))
        .map(|s| HelpVisitRecord {
            route: s.route.to_string(),
            feature_highlight: s.feature_highlight.to_string(),
            spotlight: s.spotlight.map(str::to_string),
            replay: false,
        })
        .collect();
    serde_json::to_string(&visits).unwrap_or_else(|_| "[]".to_string())
}

pub async fn seed_data(
    session: tower_sessions::Session,
    Json(body): Json<SeedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let kind = E2eAuthKind::parse(&body.auth);
    write_e2e_auth_kind(&session, kind)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fixtures = e2e_fixtures();
    let help_seen_json = if body.help_tour {
        None
    } else {
        Some(photon_help_seen_json())
    };

    Ok(Json(serde_json::json!({
        "ok": true,
        "auth": kind.as_str(),
        "fixtures": {
            "topic_name": fixtures.topic_name,
            "subscription_id": fixtures.subscription_id,
            "event_id": fixtures.event_id,
        },
        "help_seen_json": help_seen_json,
    })))
}
