//! Spotlight steps for the Topics catalog (`/photon/topics`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro for the topics catalog.
#[help_spotlight_step(
    route = "/photon/topics",
    feature_highlight = "photon-topics-intro",
    title = "Topics catalog",
    order = 10
)]
#[component]
pub fn PhotonTopicsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topics-intro",
        "Topics is the catalog of channels. Each card is one named place where messages are published.",
        Some("KEYED means messages are sorted into slices by a key (like sorting mail by address)."),
        &[],
    )
}

/// Search box.
#[help_spotlight_step(
    route = "/photon/topics",
    feature_highlight = "photon-topics-search",
    title = "Find a topic",
    spotlight = "photon-topics-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn PhotonTopicsSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topics-search",
        "Type part of a topic name or schema text. The list narrows as you type. Clear the box to see everything.",
        None,
        &[],
    )
}

/// Topic card fields.
#[help_spotlight_step(
    route = "/photon/topics",
    feature_highlight = "photon-topics-card",
    title = "What a topic shows",
    spotlight = "photon-topics-card",
    position = "top",
    order = 30
)]
#[component]
pub fn PhotonTopicsCardHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topics-card",
        "On each card you can read these fields.",
        None,
        &[
            "Name (+ KEYED): channel id and whether it slices",
            "Keyed by: which field defines the slice",
            "Schema: shape of messages on this channel",
            "Events (24h): recent message volume",
            "Subscriptions: how many listeners are attached",
        ],
    )
}

/// View button → topic detail.
#[help_spotlight_step(
    route = "/photon/topics",
    feature_highlight = "photon-topics-view",
    title = "Open this topic",
    spotlight = "photon-topics-btn-view",
    position = "top",
    order = 40
)]
#[component]
pub fn PhotonTopicsViewHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topics-view",
        "View (or the card body) opens this topic's detail: schema, its listeners, and recent messages for this channel only.",
        None,
        &[],
    )
}

/// View Events → global events index.
#[help_spotlight_step(
    route = "/photon/topics",
    feature_highlight = "photon-topics-view-events",
    title = "Jump to Events",
    spotlight = "photon-topics-btn-view-events",
    position = "top",
    order = 50
)]
#[component]
pub fn PhotonTopicsViewEventsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topics-view-events",
        "Opens the Events page for all channels. Use the topic filter there if you want only this channel's messages.",
        None,
        &[],
    )
}

/// View Subscriptions → global subscriptions index.
#[help_spotlight_step(
    route = "/photon/topics",
    feature_highlight = "photon-topics-view-subs",
    title = "Jump to Subscriptions",
    spotlight = "photon-topics-btn-view-subs",
    position = "top",
    order = 60
)]
#[component]
pub fn PhotonTopicsViewSubsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topics-view-subs",
        "Opens the Subscriptions page for every listener. Search or filter ON/OFF there to find who is attached.",
        None,
        &[],
    )
}
