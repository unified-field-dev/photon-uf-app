//! Spotlight steps for subscription detail (`/photon/subscriptions/:id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Subscription metadata.
#[help_spotlight_step(
    route = "/photon/subscriptions/:id",
    feature_highlight = "photon-sub-meta",
    title = "This subscription",
    spotlight = "photon-sub-meta",
    position = "bottom",
    order = 10
)]
#[component]
pub fn PhotonSubMetaHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-sub-meta",
        "You are looking at one listener.",
        None,
        &[
            "Name + ON/OFF: which listener and whether live",
            "Topic: channel it reads",
            "Mode: how delivery runs",
            "Key filter: which slices it accepts ((all))",
            "Checkpoint lag: how far behind the tip it is",
            "Last seq: last sequence it processed",
        ],
    )
}

/// Recent events on the subscription's topic.
#[help_spotlight_step(
    route = "/photon/subscriptions/:id",
    feature_highlight = "photon-sub-events",
    title = "Recent messages on its topic",
    spotlight = "photon-sub-events",
    position = "top",
    order = 20
)]
#[component]
pub fn PhotonSubEventsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-sub-events",
        "These rows are from the listener's topic (not only messages this listener already processed).",
        None,
        &[
            "Event ID: open payload and actor",
            "Seq / Created: order and time",
        ],
    )
}
