//! Spotlight steps for topic detail (`/photon/topics/:topic_name`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Topic metadata card.
#[help_spotlight_step(
    route = "/photon/topics/:topic_name",
    feature_highlight = "photon-topic-meta",
    title = "This topic",
    spotlight = "photon-topic-meta",
    position = "bottom",
    order = 10
)]
#[component]
pub fn PhotonTopicMetaHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topic-meta",
        "You are looking at one channel.",
        None,
        &[
            "Keyed by: slice field, or \"-\" if unkeyed",
            "Schema: message shape for this channel",
            "Events (24h): recent volume on this channel",
            "Subscriptions: listeners attached here",
        ],
    )
}

/// Subscriptions for this topic.
#[help_spotlight_step(
    route = "/photon/topics/:topic_name",
    feature_highlight = "photon-topic-subs",
    title = "Listeners on this channel",
    spotlight = "photon-topic-subs",
    position = "top",
    order = 20
)]
#[component]
pub fn PhotonTopicSubsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topic-subs",
        "Use Name when you need lag or key-filter details.",
        None,
        &[
            "Name: click to open that listener's page",
            "Enabled: Yes means it is ON; No means OFF",
        ],
    )
}

/// Recent events for this topic.
#[help_spotlight_step(
    route = "/photon/topics/:topic_name",
    feature_highlight = "photon-topic-events",
    title = "Recent messages here",
    spotlight = "photon-topic-events",
    position = "top",
    order = 30
)]
#[component]
pub fn PhotonTopicEventsHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-topic-events",
        "These rows are messages on this channel only.",
        None,
        &[
            "Event ID: click to open payload and actor",
            "Seq: order on this channel",
            "Created: when it was published",
        ],
    )
}
