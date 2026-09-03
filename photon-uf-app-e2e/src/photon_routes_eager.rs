//! Eager `/photon` routes for the Playwright host.
//!
//! Production [`photon_app::PhotonRoutes`] wraps leaf pages in `Lazy` for
//! wasm-split. Nested `Lazy` under `ParentRoute` still panics on
//! `hydrate_body` in this Leptos pin, so the lab host mounts the same page
//! components without `Lazy`.

use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path,
};
use photon_app::{
    PhotonDashboardPage, PhotonEventDetailPage, PhotonEventsIndexPage, PhotonLayout,
    PhotonSubscriptionDetailPage, PhotonSubscriptionsIndexPage, PhotonTopicDetailPage,
    PhotonTopicsIndexPage,
};

/// Same paths as [`photon_app::PhotonRoutes`], without Lazy route views.
#[component(transparent)]
pub fn PhotonRoutesEager() -> impl leptos_router::MatchNestedRoutes + Clone {
    photon_app::ensure_help_steps_linked();
    view! {
        <ParentRoute path=path!("photon") view=PhotonLayout>
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
