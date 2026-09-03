//! Spotlight steps for the Subscriptions catalog (`/photon/subscriptions`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro for subscriptions.
#[help_spotlight_step(
    route = "/photon/subscriptions",
    feature_highlight = "photon-subs-intro",
    title = "Subscriptions catalog",
    order = 10
)]
#[component]
pub fn PhotonSubsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-subs-intro",
        "A subscription is a listener tuned to a topic. Think of it as a radio set to one station — it keeps a bookmark (checkpoint) of how far it has read.",
        None,
        &[],
    )
}

/// Search field.
#[help_spotlight_step(
    route = "/photon/subscriptions",
    feature_highlight = "photon-subs-search",
    title = "Find a listener",
    spotlight = "photon-subs-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn PhotonSubsSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-subs-search",
        "Type part of a listener name or topic name. The list narrows as you type.",
        None,
        &[],
    )
}

/// Status filter.
#[help_spotlight_step(
    route = "/photon/subscriptions",
    feature_highlight = "photon-subs-status",
    title = "Filter ON or OFF",
    spotlight = "photon-subs-status",
    position = "bottom",
    order = 30
)]
#[component]
pub fn PhotonSubsStatusHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-subs-status",
        "Choose All, ON, or OFF to show every listener, only live ones, or only idle ones.",
        None,
        &[],
    )
}

/// Card fields.
#[help_spotlight_step(
    route = "/photon/subscriptions",
    feature_highlight = "photon-subs-card",
    title = "What a subscription shows",
    spotlight = "photon-subs-card",
    position = "top",
    order = 40
)]
#[component]
pub fn PhotonSubsCardHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-subs-card",
        "Each card summarizes one listener.",
        None,
        &[
            "Name + ON/OFF: which listener and whether it is live",
            "Topic: channel it reads",
            "Mode: how delivery runs",
            "Key filter: which slices it accepts ((all) means every slice)",
            "Checkpoint lag: how far behind the tip it is",
            "Last processed: when it last advanced",
        ],
    )
}

/// Open card → detail.
#[help_spotlight_step(
    route = "/photon/subscriptions",
    feature_highlight = "photon-subs-open",
    title = "Open this subscription",
    spotlight = "photon-subs-card",
    position = "top",
    order = 50
)]
#[component]
pub fn PhotonSubsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-subs-open",
        "Click the card to open this listener's detail page: configuration, lag, and recent messages on its topic.",
        None,
        &[],
    )
}
