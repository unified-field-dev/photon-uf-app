//! Spotlight steps for event detail (`/photon/events/:id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Event metadata fields.
#[help_spotlight_step(
    route = "/photon/events/:id",
    feature_highlight = "photon-event-meta",
    title = "This message",
    spotlight = "photon-event-meta",
    position = "bottom",
    order = 10
)]
#[component]
pub fn PhotonEventMetaHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-event-meta",
        "These fields identify the message. If you see \"Transport payload expired\", the body is gone and only metadata remains.",
        None,
        &[
            "Topic: which channel",
            "Key: slice (or a dash if none)",
            "Seq: order on that channel",
            "Created: when it was published",
            "Status: delivery state",
        ],
    )
}

/// Payload JSON.
#[help_spotlight_step(
    route = "/photon/events/:id",
    feature_highlight = "photon-event-payload",
    title = "Payload",
    spotlight = "photon-event-payload",
    position = "top",
    order = 20
)]
#[component]
pub fn PhotonEventPayloadHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-event-payload",
        "Payload is the message body as JSON. Read it here when you need to see what was published.",
        None,
        &[],
    )
}

/// Actor JSON.
#[help_spotlight_step(
    route = "/photon/events/:id",
    feature_highlight = "photon-event-actor",
    title = "Actor",
    spotlight = "photon-event-actor",
    position = "top",
    order = 30
)]
#[component]
pub fn PhotonEventActorHelp() -> impl IntoView {
    help_stack(
        "help-step-photon-event-actor",
        "Actor is context about who or what published the message, also as JSON.",
        None,
        &[],
    )
}
