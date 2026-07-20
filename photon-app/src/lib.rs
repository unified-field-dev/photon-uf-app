#![recursion_limit = "256"]
//! Photon operations app: routes and UI composition for inspecting Photon topics,
//! subscriptions, and event streams under `/photon`.
//!
//! Photon itself is an event-pipeline crate with no built-in UI; this crate is the
//! `#[uf_product_macros::orbital_app]`-registered operations surface a host mounts to give
//! operators visibility into what Photon is doing at runtime.
//!
//! Orbital inventory macros (`orbital_app!`, `orbital_routes_extract`) emit undocumented
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
//! ## Getting started
//!
//! Mount [`PhotonRoutes`] inside your host's `<Routes>`; it registers the `/photon` subtree
//! (auth-gated) and, via `orbital_app!`, its launcher metadata:
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
use leptos_router::{components::*, path};
use uf_product_macros::orbital_app;

mod components;
mod layout;
/// Page components for the Photon ops UI.
pub mod pages;
/// SSR server functions and DTOs backing the Photon ops UI.
pub mod server;

pub use layout::PhotonLayout;
pub use pages::{
    PhotonDashboardPage, PhotonEventDetailPage, PhotonEventsIndexPage,
    PhotonSubscriptionDetailPage, PhotonSubscriptionsIndexPage, PhotonTopicDetailPage,
    PhotonTopicsIndexPage,
};
pub use server::{DashboardStats, EventSummary, SubscriptionSummary, TopicSummary};

#[component]
fn PhotonAuthGuard() -> impl IntoView {
    view! {
        <orbital::routes::RequireAuthenticated>
            <PhotonLayout />
        </orbital::routes::RequireAuthenticated>
    }
}

orbital_app! {
    name: "Photon",
    id: "photon",
    description: "Event pipeline management",
    icon: "💫",
    version: "0.1.0",
    routes: PhotonRoutes,
    route_path: "/photon",
}

/// Photon's nested route tree, gated behind an auth guard and mounted at `/photon`.
///
/// Registers dashboard, topic, subscription, and event index/detail routes. Intended to be
/// used inside a host `<Routes>` component, e.g. `<PhotonRoutes />`.
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn PhotonRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("photon") view=PhotonAuthGuard>
            <Route path=path!("") view=PhotonDashboardPage />
            <Route path=path!("topics") view=PhotonTopicsIndexPage />
            <Route path=path!("topics/:topic_name") view=PhotonTopicDetailPage />
            <Route path=path!("subscriptions") view=PhotonSubscriptionsIndexPage />
            <Route path=path!("subscriptions/:id") view=PhotonSubscriptionDetailPage />
            <Route path=path!("events") view=PhotonEventsIndexPage />
            <Route path=path!("events/:id") view=PhotonEventDetailPage />
        </ParentRoute>
    }
    .into_inner()
}
