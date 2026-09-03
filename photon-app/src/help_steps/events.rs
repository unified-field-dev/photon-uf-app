//! Spotlight steps for the Events catalog (`/photon/events`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro for events.
#[help_spotlight_step(
    route = "/photon/events",
    feature_highlight = "photon-events-intro",
    title = "Events history",
    order = 10
)]
#[component]
pub fn PhotonEventsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-events-intro",
        "Events is the history of published messages. Each row is one message you can open and inspect.",
        None,
        &[],
    )
}

/// Topic filter.
#[help_spotlight_step(
    route = "/photon/events",
    feature_highlight = "photon-events-filter",
    title = "Filter by topic",
    spotlight = "photon-events-filter",
    position = "bottom",
    order = 20
)]
#[component]
pub fn PhotonEventsFilterHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-events-filter",
        "Choose Topic: All for every channel, or pick one topic to narrow the list.",
        None,
        &[],
    )
}

/// Events table.
#[help_spotlight_step(
    route = "/photon/events",
    feature_highlight = "photon-events-table",
    title = "Browse messages",
    spotlight = "photon-events-table",
    position = "top",
    order = 30
)]
#[component]
pub fn PhotonEventsTableHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-events-table",
        "Scan the list, then open a message for payload and actor detail.",
        None,
        &[
            "Event ID: click to open the message",
            "Topic: which channel",
            "Key: slice on that channel (if any)",
            "Seq: order number",
            "Created: when it was published",
        ],
    )
}
