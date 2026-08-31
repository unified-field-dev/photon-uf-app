//! Leptos server functions for Photon UI.
//!
//! DTOs and pure mapping helpers live in [`photon_backend`] so contracts stay
//! unit/integration-testable without the host UI graph. Photon IO for ops reads
//! lives in [`photon_backend::ops`] (feature `ops`). Server functions run on
//! SSR only and use Photon request context for IO (Chronon/Boson-shaped: no
//! Valence ops projection).
//!
//! ## Security map
//!
//! - Every ops-UI server fn requires an authenticated session and
//!   `PhotonAdmin` (via `#[uf_product_macros::server(permission = "...")]`).
//! - Catalog, subscription, and event reads come from Photon (`admin_snapshot`,
//!   registry, `list_*`, `get_event`).
//! - Path/query ids are rejected when blank, oversized, or containing `/` `\`,
//!   controls, or `.` / `..` (`photon_backend::validate_*`).
//!
//! ## Errors
//!
//! Fallible ops return [`ServerFnError`] (Leptos boundary). Blank path/query ids
//! are rejected by `photon_backend::validate_*` as [`photon_backend::PhotonIdError`]
//! and mapped with operation context. Missing session, missing Photon context, and
//! Photon IO failures are also `ServerFnError` strings at this boundary.

use leptos::prelude::*;
pub use photon_backend::{
    DashboardStats, EventDetail, EventSummary, SubscriptionSummary, TopicSummary,
};

/// Permission name required for Photon admin reads
/// (manifest: [`crate::permissions::PhotonPermission::PhotonAdmin`]).
pub const PHOTON_ADMIN_PERMISSION: &str = "PhotonAdmin";

#[cfg(feature = "ssr")]
fn map_ops_err(e: photon_backend::ops::OpsError) -> ServerFnError {
    ServerFnError::new(e.to_string())
}

#[cfg(feature = "ssr")]
fn photon_from_context() -> Result<std::sync::Arc<photon::Photon>, ServerFnError> {
    leptos::context::use_context::<std::sync::Arc<photon::Photon>>()
        .ok_or_else(|| ServerFnError::new("Photon not in request context"))
}

#[cfg(feature = "ssr")]
fn require_session(ctx: &higgs::Higgs) -> Result<(), ServerFnError> {
    photon_backend::ops::require_session_user(ctx.session_user_id().map(String::as_str))
        .map_err(map_ops_err)
}

// ============================================================================
// Server functions (SSR only)
// ============================================================================

/// Get dashboard stats (topic count, subscription count, event count 24h).
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_dashboard_stats(&photon)
        .await
        .map_err(map_ops_err)
}

/// Get recent events for dashboard.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_recent_events(
    /// Maximum number of recent events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_recent_events(&photon, limit)
        .await
        .map_err(map_ops_err)
}

/// Get all topics (from registry + Photon list counts).
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_topics() -> Result<Vec<TopicSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_topics(&photon)
        .await
        .map_err(map_ops_err)
}

/// Get a single topic by name.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_topic(
    /// Name of the topic to look up.
    topic_name: String,
) -> Result<Option<TopicSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_topic(&photon, &topic_name)
        .await
        .map_err(map_ops_err)
}

/// Get all subscriptions (handler inventory + checkpoints via `admin_snapshot`).
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_subscriptions() -> Result<Vec<SubscriptionSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::list_subscriptions(&photon)
        .await
        .map_err(map_ops_err)
}

/// Get a single subscription by ID.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_subscription(
    /// Unique identifier of the subscription to look up.
    id: String,
) -> Result<Option<SubscriptionSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_subscription(&photon, &id)
        .await
        .map_err(map_ops_err)
}

/// Get events (recent across all topics, or optionally filter by topic).
///
/// Results are capped at [`photon_backend::MAX_EVENT_LIST_LIMIT`]. When
/// `topic_name` is set, the query is topic-scoped via Photon `list_events_by_topic`.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_events(
    /// Optional topic name to restrict results to; when omitted, events from all
    /// topics are considered (still capped).
    topic_name: Option<String>,
    /// Maximum number of events to return.
    limit: u32,
) -> Result<Vec<EventSummary>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_events(&photon, topic_name.as_deref(), limit)
        .await
        .map_err(map_ops_err)
}

/// Get a single event by ID.
#[uf_product_macros::server(permission = "PhotonAdmin")]
pub async fn get_event(
    /// Unique identifier of the event to look up.
    id: String,
) -> Result<Option<EventDetail>, ServerFnError> {
    let ctx = higgs::Higgs::from_request().await?;
    require_session(&ctx)?;
    let photon = photon_from_context()?;
    photon_backend::ops::load_event(&photon, &id)
        .await
        .map_err(map_ops_err)
}
