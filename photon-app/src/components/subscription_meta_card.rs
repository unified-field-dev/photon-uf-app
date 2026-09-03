//! Subscription metadata card for the subscription detail page.

use leptos::prelude::*;
use orbital::components::Body1;

use crate::components::SubscriptionStatusBadge;
use crate::server::SubscriptionSummary;
use orbital::components::Card;

/// Renders a card with subscription metadata: name, status badge, topic, mode,
/// key filter, checkpoint lag, and last seq.
#[component]
pub fn SubscriptionMetaCard(
    /// Subscription data to display.
    sub: SubscriptionSummary,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Meta { color: var(--orb-color-text-tertiary); margin-bottom: 8px; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <Body1 block=true class=class_names.meta>
                "Name: " {sub.subscription_name.clone()}
                " "
                <SubscriptionStatusBadge enabled=sub.enabled />
            </Body1>
            <Body1 block=true class=class_names.meta>"Topic: " {sub.topic_name.clone()}</Body1>
            <Body1 block=true class=class_names.meta>"Mode: " {sub.mode.clone()}</Body1>
            <Body1 block=true class=class_names.meta>"Key filter: " {sub.topic_key_filter.clone().unwrap_or_else(|| "(all)".to_string())}</Body1>
            <Body1 block=true class=class_names.meta>"Checkpoint lag: " {sub.checkpoint_lag}</Body1>
            {sub.last_seq.map(|seq| view! { <Body1 block=true class=class_names.meta>"Last seq: " {seq}</Body1> })}
        </Card>
    }
}
