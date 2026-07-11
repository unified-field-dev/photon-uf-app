#![recursion_limit = "256"]
//! Photon operations app routes and UI composition.
//!
//! This app provides the operational UI for inspecting Photon topics,
//! subscriptions, and event streams under `/photon`.
//!
//! ## UI features
//!
//! - Dashboard cards for topic/subscription/event activity.
//! - Topic index/detail views.
//! - Subscription index/detail views.
//! - Event index/detail views with payload previews.
//!
//! ## What it manages
//!
//! - Runtime visibility into topic descriptors and traffic.
//! - Subscription health/read-state visibility.
//! - Event inspection for debugging and operations.
//!
//! ## Backend API surface
//!
//! The app's server module provides read APIs for dashboard stats, topics,
//! subscriptions, and events in [`server`].
//!
//! Route entrypoint: [`PhotonRoutes`].

use leptos::prelude::*;
use leptos_router::{components::*, path};
use uf_product_macros::orbital_app;

mod components;
mod layout;
mod pages;
mod server;

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
