#![recursion_limit = "256"]
//! Photon operations app: routes and UI composition for inspecting Photon topics,
//! subscriptions, and event streams under `/photon`.
//!
//! Photon itself is an event-pipeline crate with no built-in UI; this crate is the
//! `#[uf_product_macros::uf_app]`-registered operations surface a host mounts to give
//! operators visibility into what Photon is doing at runtime.
//!
//! Orbital inventory macros (`uf_app!`, `orbital_routes_extract`) emit undocumented
//! associated items, so this crate allows `missing_docs` at the crate root while keeping
//! hand-written modules and items documented.
//!
//! ## Features
//!
//! - **Dashboard** — [`PhotonDashboardPage`] shows aggregate topic/subscription/event
//!   activity at a glance.
//! - **Topics** — [`PhotonTopicsIndexPage`] / [`PhotonTopicDetailPage`] for browsing topic
//!   schemas, keying, and traffic.
//! - **Subscriptions** — [`PhotonSubscriptionsIndexPage`] / [`PhotonSubscriptionDetailPage`]
//!   for subscription configuration and checkpoint/read-state visibility.
//! - **Events** — [`PhotonEventsIndexPage`] / [`PhotonEventDetailPage`] for inspecting
//!   individual events, including payload previews and actor context.
//! - **Read API** — [`mod@server`] exposes the SSR-only server functions and DTOs
//!   ([`DashboardStats`], [`TopicSummary`], [`SubscriptionSummary`], [`EventSummary`]) backing
//!   the pages above.
//!
//! ## Routes
//!
//! Mounted under `/photon` by [`PhotonRoutes`]. Concern → page → key server fn(s):
//!
//! | Path | Page | Key server fn(s) |
//! |---|---|---|
//! | `/photon` | [`PhotonDashboardPage`] | `get_dashboard_stats`, `get_recent_events` |
//! | `/photon/topics` | [`PhotonTopicsIndexPage`] | `get_topics` |
//! | `/photon/topics/:topic_name` | [`PhotonTopicDetailPage`] | `get_topic`, `get_subscriptions`, `get_events` |
//! | `/photon/subscriptions` | [`PhotonSubscriptionsIndexPage`] | `get_subscriptions` |
//! | `/photon/subscriptions/:id` | [`PhotonSubscriptionDetailPage`] | `get_subscription`, `get_events` |
//! | `/photon/events` | [`PhotonEventsIndexPage`] | `get_events` |
//! | `/photon/events/:id` | [`PhotonEventDetailPage`] | `get_event` |
//!
//! All routes are read-only today; there are no create/edit flows in this UI.
//!
//! ## Getting started
//!
//! Mount [`PhotonRoutes`] inside your host's `<Routes>`; it registers the `/photon` subtree
//! (auth-gated) and, via `uf_app!`, its launcher metadata:
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_router::components::Routes;
//! use photon_app::PhotonRoutes;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Routes fallback=|| "not found">
//!             <PhotonRoutes />
//!         </Routes>
//!     }
//! }
//! ```
//!
//! ## Where to look next
//!
//! - [`PhotonRoutes`] — the route entrypoint mounted by hosts.
//! - [`PhotonLayout`] — the shared app bar / nav shell wrapping every route.
//! - [`mod@server`] — server functions and DTOs backing the UI.

#![allow(missing_docs)]
#![cfg_attr(
    feature = "ssr",
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        unknown_lints,
        clippy::all,
    )
)]

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
pub use server::{DashboardStats, EventSummary, SubscriptionSummary, TopicSummary};

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
