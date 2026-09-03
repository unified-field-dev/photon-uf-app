//! Lazy-loaded route views for WASM code-splitting (`cargo leptos --split`).
//!
//! `LazyRoute::view` takes `Self` by value; leaf pages ignore it (trait shape).

#![allow(clippy::used_underscore_binding)]

use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::{
    PhotonDashboardPage, PhotonEventDetailPage, PhotonEventsIndexPage, PhotonLayout,
    PhotonSubscriptionDetailPage, PhotonSubscriptionsIndexPage, PhotonTopicDetailPage,
    PhotonTopicsIndexPage,
};

/// Prefetch the photon family WASM chunk (leaf pages share split modules).
pub async fn prefetch_family() {
    PhotonDashboardRoute::preload().await;
}

/// Eager layout shell for `/photon/*` ParentRoute (auth gate lives inside [`PhotonLayout`]).
#[component]
pub fn PhotonLayoutRouteView() -> impl IntoView {
    view! { <PhotonLayout /> }
}

/// Lazy `/photon` dashboard.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonDashboardRoute;

#[lazy_route]
impl LazyRoute for PhotonDashboardRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonDashboardPage /> }.into_any()
    }
}

/// Lazy `/photon/topics`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonTopicsIndexRoute;

#[lazy_route]
impl LazyRoute for PhotonTopicsIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonTopicsIndexPage /> }.into_any()
    }
}

/// Lazy `/photon/topics/:topic_name`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonTopicDetailRoute;

#[lazy_route]
impl LazyRoute for PhotonTopicDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonTopicDetailPage /> }.into_any()
    }
}

/// Lazy `/photon/subscriptions`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonSubscriptionsIndexRoute;

#[lazy_route]
impl LazyRoute for PhotonSubscriptionsIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonSubscriptionsIndexPage /> }.into_any()
    }
}

/// Lazy `/photon/subscriptions/:id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonSubscriptionDetailRoute;

#[lazy_route]
impl LazyRoute for PhotonSubscriptionDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonSubscriptionDetailPage /> }.into_any()
    }
}

/// Lazy `/photon/events`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonEventsIndexRoute;

#[lazy_route]
impl LazyRoute for PhotonEventsIndexRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonEventsIndexPage /> }.into_any()
    }
}

/// Lazy `/photon/events/:id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhotonEventDetailRoute;

#[lazy_route]
impl LazyRoute for PhotonEventDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <PhotonEventDetailPage /> }.into_any()
    }
}
