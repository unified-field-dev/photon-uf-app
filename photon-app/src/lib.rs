#![recursion_limit = "256"]
//! Photon operations app — inspect Photon topics, subscriptions, and event streams.
//!
//! Leptos UI mounted under `/photon` so operators can see what Photon is doing at
//! runtime without building custom pages. Registers alongside other product apps via
//! `uf_app!` and requires an authenticated session with `PhotonAdmin` before server
//! functions load registry or event data.
//!
//! Orbital inventory macros (`uf_app!`, `orbital_routes_extract`) emit undocumented
//! associated items, so this crate allows `missing_docs` at the crate root while keeping
//! hand-written modules and items documented.
//!
//! ## Features
//!
//! - **Photon admin routes** — Provides the nested `/photon` route tree behind auth for
//!   dashboard, topics, subscriptions, and events. Mount once when the host router starts.
//!   [Get started](#mount-photon-routes)
//! - **Dashboard KPIs** — Shows topic, subscription, and 24-hour event counters on
//!   [`PhotonDashboardPage`] via [`get_dashboard_stats`] plus a recent-events preview
//!   table from [`get_recent_events`]. [Get started](#dashboard-kpis)
//! - **Topics browser** — Lists registry topics and opens detail pages via [`get_topics`]
//!   and [`get_topic`]. [Get started](#browse-topics)
//! - **Subscriptions browser** — Lists delivery handlers and opens detail pages via
//!   [`get_subscriptions`] and [`get_subscription`].
//!   [Get started](#browse-subscriptions)
//! - **Events browser** — Lists stored transport events and opens detail pages via
//!   [`get_events`] and [`get_event`]. [Get started](#browse-events)
//! - **Server function wrappers** — Exposes [`mod@server`] Higgs `#[server]` fns and DTO
//!   re-exports backed by [`photon_backend`] pure mapping helpers.
//!
//! ## Mount Photon routes
//!
//! [`PhotonRoutes`] nests the full `/photon` subtree inside a host Leptos `<Routes>` tree.
//! Operators get read-only visibility into Photon registry topics, delivery subscriptions,
//! and stored events. Mount during host router setup at startup, alongside other `uf_app!`
//! product routes — the macro registers launcher metadata and the `/photon` inventory entry.
//!
//! **Prerequisites:** `ssr` on this crate; authenticated session; `PhotonAdmin` permission
//! ([`PHOTON_ADMIN_PERMISSION`]); `Arc<photon::Photon>` in Leptos request context for IO.
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::Routes;
//! use photon_app::PhotonRoutes;
//!
//! view! {
//!     <Routes fallback=|| "not found">
//!         <PhotonRoutes />
//!     </Routes>
//! }
//! ```
//!
//! On success `/photon` resolves to the dashboard, `/photon/topics` lists registry topics,
//! and nested detail routes load topic, subscription, and event pages. Unauthenticated
//! sessions are rejected by server functions — see root `SECURITY.md`.
//!
//! ## Dashboard KPIs
//!
//! The dashboard answers how large the Photon footprint is right now: topic count,
//! subscription count, and 24-hour event volume. [`PhotonDashboardPage`] calls
//! [`get_dashboard_stats`] on each SSR render and [`get_recent_events`] for the preview
//! table — use this landing page after mounting routes when operators need a quick health
//! snapshot.
//!
//! **Prerequisites:** [`PhotonRoutes`] mounted; `ssr` feature; `PhotonAdmin` permission;
//! Photon request context wired.
//!
//! ```rust,ignore
//! use photon_app::{
//!     PhotonDashboardPage, get_dashboard_stats, get_recent_events, DashboardStats,
//! };
//!
//! // PhotonDashboardPage calls these on each SSR render:
//! let stats: DashboardStats = get_dashboard_stats().await?;
//! assert_eq!(stats.topic_count, 3);
//! assert_eq!(stats.event_count_24h, 12);
//!
//! let recent = get_recent_events(20).await?;
//! assert!(recent.len() <= 20);
//! ```
//!
//! On success `stats` carries `topic_count`, `subscription_count`, and `event_count_24h`;
//! `recent` holds preview rows for the dashboard table. Blank or unsafe path ids are
//! rejected by `photon_backend::validate_*` before Photon IO.
//!
//! ## Browse topics
//!
//! Topic pages list registry schemas and per-topic subscription counts. [`PhotonTopicsIndexPage`]
//! loads [`get_topics`] for the index; [`PhotonTopicDetailPage`] calls [`get_topic`] and
//! filtered subscriptions and events for one topic name. Open these routes when operators
//! need schema JSON or a single-topic drill-down.
//!
//! **Prerequisites:** Routes mounted; topic names must pass `photon_backend::validate_topic_name`.
//!
//! ```rust,ignore
//! use photon_app::{
//!     PhotonTopicsIndexPage, get_topics, get_topic, TopicSummary,
//! };
//!
//! // PhotonTopicsIndexPage loads get_topics for the index:
//! let topics: Vec<TopicSummary> = get_topics().await?;
//! assert_eq!(topics.first().map(|t| t.topic_name.as_str()), Some("orders"));
//!
//! let detail = get_topic("orders".into()).await?;
//! assert_eq!(detail.topic_name, "orders");
//! ```
//!
//! On success the index returns sorted [`TopicSummary`] rows and detail resolves one topic
//! or maps a missing name to a server error. Oversized or path-unsafe names fail validation
//! before registry lookup.
//!
//! ## Browse subscriptions
//!
//! Subscription pages show delivery handlers, checkpoint lag, and topic attachment.
//! [`PhotonSubscriptionsIndexPage`] loads [`get_subscriptions`]; [`PhotonSubscriptionDetailPage`]
//! calls [`get_subscription`] plus filtered events for one handler id. Use these routes
//! when operators trace which consumers are attached to a topic or inspect lag.
//!
//! **Prerequisites:** Routes mounted; subscription ids must pass
//! `photon_backend::validate_subscription_id`.
//!
//! ```rust,ignore
//! use photon_app::{
//!     PhotonSubscriptionsIndexPage, get_subscriptions, get_subscription,
//!     SubscriptionSummary,
//! };
//!
//! // PhotonSubscriptionsIndexPage loads get_subscriptions:
//! let subs: Vec<SubscriptionSummary> = get_subscriptions().await?;
//! assert_eq!(subs.first().map(|s| s.subscription_id.as_str()), Some("reg.key"));
//!
//! let detail = get_subscription("reg.key".into()).await?;
//! assert_eq!(detail.subscription_id, "reg.key");
//! ```
//!
//! On success the index lists [`SubscriptionSummary`] rows from the admin snapshot and detail
//! resolves one handler or errors when the id is unknown. Blank or slash-containing ids are
//! rejected before lookup.
//!
//! ## Browse events
//!
//! Event pages list stored transport rows and full payload JSON on detail. [`PhotonEventsIndexPage`]
//! loads [`get_events`] with a capped limit; [`PhotonEventDetailPage`] calls [`get_event`] for
//! one id. Open these routes when operators audit delivery status or inspect payload bodies.
//!
//! **Prerequisites:** Routes mounted; event ids must pass `photon_backend::validate_event_id`;
//! list limits are capped by `photon_backend::clamp_event_list_limit`.
//!
//! ```rust,ignore
//! use photon_app::{
//!     PhotonEventsIndexPage, get_events, get_event, EventDetail,
//! };
//!
//! // PhotonEventsIndexPage loads get_events with a capped limit:
//! let rows = get_events(50).await?;
//! assert!(rows.len() <= 50);
//!
//! let detail: EventDetail = get_event("ev-1".into()).await?;
//! assert_eq!(detail.event_id, "ev-1");
//! ```
//!
//! On success the index returns [`EventSummary`] preview rows and detail returns full
//! [`EventDetail`] including payload JSON when transport retention allows. Expired transport
//! payloads surface `transport_expired` on detail without failing the page shell.
//!
//! ## Feature flags
//!
//! | Flag | Effect |
//! |------|--------|
//! | `ssr` | Server-side Leptos split; required for `#[server]` fns and Photon IO. |
//! | `hydrate` | Client-side hydration for routed pages and Orbital shell components. |
//!
//! ## Routes
//!
//! Mounted under `/photon` by [`PhotonRoutes`]. All routes are read-only today.
//!
//! | Path | Page | Key server fn(s) |
//! |---|---|---|
//! | `/photon` | [`PhotonDashboardPage`] | [`get_dashboard_stats`], [`get_recent_events`] |
//! | `/photon/topics` | [`PhotonTopicsIndexPage`] | [`get_topics`] |
//! | `/photon/topics/:topic_name` | [`PhotonTopicDetailPage`] | [`get_topic`], [`get_subscriptions`], [`get_events`] |
//! | `/photon/subscriptions` | [`PhotonSubscriptionsIndexPage`] | [`get_subscriptions`] |
//! | `/photon/subscriptions/:id` | [`PhotonSubscriptionDetailPage`] | [`get_subscription`], [`get_events`] |
//! | `/photon/events` | [`PhotonEventsIndexPage`] | [`get_events`] |
//! | `/photon/events/:id` | [`PhotonEventDetailPage`] | [`get_event`] |
//!
//! ## Examples
//!
//! Start with [Mount Photon routes](#mount-photon-routes). The `photon-backend` unit and integ
//! suites in `docs/VERIFICATION.md` cover server-fn contracts. Runnable host:
//! `examples/protected-photon-host` (auth + dashboard KPIs; inventory `photon` / `/photon`).
//!
//! ## Where to look next
//!
//! - [`PhotonLayout`] — shared app bar / nav shell wrapping every route.
//! - [`mod@server`] — server functions and DTOs backing the UI.
//! - [`permissions::PhotonPermission`] — permission manifest for `PhotonAdmin`.
//! - `photon_backend` — id validation and pure mapping helpers used by these server fns.

#![allow(missing_docs)]
// Orbital / Leptos macros leave cfg-gated items that look unused under `ssr` alone;
// keep the allow narrow to unknown_lints (workspace also allows unknown_lints).
#![cfg_attr(feature = "ssr", allow(unknown_lints))]

use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path, Lazy,
};
use uf_product_macros::uf_app;

mod components;
mod layout;
mod lazy_routes;
/// Page components for the Photon ops UI.
pub mod pages;
/// Permission manifest for Photon admin server functions.
pub mod permissions;
/// SSR server functions and DTOs backing the Photon ops UI.
pub mod server;

pub use layout::PhotonLayout;
pub use lazy_routes::{
    prefetch_family, PhotonDashboardRoute, PhotonEventDetailRoute, PhotonEventsIndexRoute,
    PhotonLayoutRouteView, PhotonSubscriptionDetailRoute, PhotonSubscriptionsIndexRoute,
    PhotonTopicDetailRoute, PhotonTopicsIndexRoute,
};
pub use pages::{
    PhotonDashboardPage, PhotonEventDetailPage, PhotonEventsIndexPage,
    PhotonSubscriptionDetailPage, PhotonSubscriptionsIndexPage, PhotonTopicDetailPage,
    PhotonTopicsIndexPage,
};
pub use server::{
    get_dashboard_stats, get_event, get_events, get_recent_events, get_subscription,
    get_subscriptions, get_topic, get_topics, DashboardStats, EventDetail, EventSummary,
    SubscriptionSummary, TopicSummary, PHOTON_ADMIN_PERMISSION,
};

uf_app! {
    name: "Photon",
    id: "photon",
    description: "Event pipeline management",
    icon: "💫",
    version: "0.1.0",
    routes: PhotonRoutes,
    route_path: "/photon",
    permission_manifest: permissions::PhotonPermission,
}

/// Photon's nested route tree, gated behind an auth guard and mounted at `/photon`.
///
/// Leaf pages are [`LazyRoute`](leptos_router::LazyRoute) views so
/// `cargo leptos --split` can emit a separate WASM chunk for this family.
/// Registers dashboard, topic, subscription, and event index/detail routes. Intended to be
/// used inside a host `<Routes>` component, e.g. `<PhotonRoutes />`.
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn PhotonRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("photon") view=PhotonLayoutRouteView>
            <Route path=path!("") view={Lazy::<PhotonDashboardRoute>::new()} />
            <Route path=path!("topics") view={Lazy::<PhotonTopicsIndexRoute>::new()} />
            <Route path=path!("topics/:topic_name") view={Lazy::<PhotonTopicDetailRoute>::new()} />
            <Route path=path!("subscriptions") view={Lazy::<PhotonSubscriptionsIndexRoute>::new()} />
            <Route path=path!("subscriptions/:id") view={Lazy::<PhotonSubscriptionDetailRoute>::new()} />
            <Route path=path!("events") view={Lazy::<PhotonEventsIndexRoute>::new()} />
            <Route path=path!("events/:id") view={Lazy::<PhotonEventDetailRoute>::new()} />
        </ParentRoute>
    }
    .into_inner()
}
