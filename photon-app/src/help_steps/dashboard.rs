//! Spotlight steps for the Photon dashboard (`/photon`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro: radio network metaphor and Topic / Subscription / Event vocabulary.
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-intro",
    title = "Welcome to Photon",
    order = 10
)]
#[component]
pub fn PhotonIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-intro",
        "Photon is the control room for live messages. Think of it like a radio network: a station broadcasts on a channel, listeners tune in, and each broadcast is one message you can inspect.",
        Some("We will walk the screens one piece at a time."),
        &[
            "Topic: a named channel for messages",
            "Subscription: a listener on a channel",
            "Event: one message that was published",
        ],
    )
}

/// KPI cards: Topics, Subscriptions, Events (24h).
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-dashboard-stats",
    title = "At a glance",
    spotlight = "photon-dashboard-stats",
    position = "bottom",
    order = 20
)]
#[component]
pub fn PhotonDashboardStatsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-dashboard-stats",
        "These three numbers are today's pulse.",
        Some("Come back here when you want a quick health check."),
        &[
            "Topics: how many channels exist",
            "Subscriptions: how many listeners exist",
            "Events (24h): messages in the last day",
        ],
    )
}

/// View All → Events.
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-ql-events",
    title = "Open all events",
    spotlight = "photon-dashboard-view-events",
    position = "top",
    order = 30
)]
#[component]
pub fn PhotonQlEventsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-ql-events",
        "This link opens the full Events list so you can filter by topic and open any message.",
        Some("Click it now, or keep touring and use the left menu later."),
        &[],
    )
}

/// Recent events preview table.
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-dashboard-recent-events",
    title = "Recent messages",
    spotlight = "photon-dashboard-recent-events",
    position = "top",
    order = 40
)]
#[component]
pub fn PhotonDashboardRecentEventsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-dashboard-recent-events",
        "This preview shows the newest messages.",
        Some("Tip: open a topic link to see payload and actor."),
        &[
            "Key: which slice of the channel (if any)",
            "Topic: which channel (click to open the event)",
            "Seq: order number on that channel",
            "Created: when it was published",
        ],
    )
}

/// View All → Subscriptions.
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-ql-subs",
    title = "Open all subscriptions",
    spotlight = "photon-dashboard-view-subs",
    position = "top",
    order = 50
)]
#[component]
pub fn PhotonQlSubsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-ql-subs",
        "This link opens every listener so you can search, filter ON/OFF, and open one.",
        None,
        &[],
    )
}

/// Active subscriptions table.
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-dashboard-active-subs",
    title = "Who is listening",
    spotlight = "photon-dashboard-active-subs",
    position = "top",
    order = 60
)]
#[component]
pub fn PhotonDashboardActiveSubsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-dashboard-active-subs",
        "Each row is a listener that is currently active.",
        Some("High lag means the listener is behind — open Name to inspect checkpoint details."),
        &[
            "Name: the listener (click to open it)",
            "Topic: which channel it reads",
            "Status: ON or OFF",
            "Lag: how far behind the newest message it is",
        ],
    )
}

/// Left navigation.
#[help_spotlight_step(
    route = "/photon",
    feature_highlight = "photon-nav",
    title = "Finding your way",
    spotlight = "photon-nav",
    position = "right",
    order = 70
)]
#[component]
pub fn PhotonNavHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-nav",
        "Use the left menu to move between Dashboard, Topics, Subscriptions, and Events.",
        Some("Help → Replay this route restarts only this page's tour."),
        &[],
    )
}
