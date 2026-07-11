//! Subscription card used in the subscriptions index page.

use leptos::prelude::*;
use leptos_router::components::A;
use orbital::components::{Body1, Body1Strong, SpacingSize};
use orbital::primitives::*;

use crate::components::SubscriptionStatusBadge;
use crate::server::SubscriptionSummary;

/// Renders a single subscription card with name, status badge, topic, mode,
/// key filter, checkpoint lag, and last processed time.
#[component]
pub fn SubscriptionCard(
    /// Subscription data to display.
    sub: SubscriptionSummary,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .SubCard {
            padding: 16px;
            border: 1px solid var(--colorNeutralStroke1);
            border-radius: 8px;
            cursor: pointer;
            background: var(--colorNeutralBackground1);
        }
        .SubCard:hover { background: var(--colorNeutralBackground1Hover); }
        .Link { text-decoration: none; color: inherit; }
        .Muted { color: var(--colorNeutralForeground3); }
    };

    let href = crate::paths::subscription(&sub.subscription_id);
    let sub_name = sub.subscription_name.clone();
    let test_id = format!("sub-{}", sub_name);
    let topic = sub.topic_name.clone();
    let mode = sub.mode.clone();
    let key_filter = sub
        .topic_key_filter
        .clone()
        .unwrap_or_else(|| "(all)".to_string());
    let lag = sub.checkpoint_lag;
    let en = sub.enabled;
    let last_view = sub.last_processed_at.clone().map(|t| {
        view! {
            <Body1 block=true class=class_names.muted>"Last processed: " {t}</Body1>
        }
    });

    view! {
        <style>{style_sheet}</style>
        <A href=href attr:class=class_names.link>
            <div
                class=class_names.sub_card
                data-testid=test_id
            >
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <Body1Strong block=true>
                        {sub_name}
                        " "
                        <SubscriptionStatusBadge enabled=en />
                    </Body1Strong>
                    <Body1 block=true class=class_names.muted>"Topic: " {topic}</Body1>
                    <Body1 block=true class=class_names.muted>"Mode: " {mode}</Body1>
                    <Body1 block=true class=class_names.muted>"Key filter: " {key_filter}</Body1>
                    <Body1 block=true class=class_names.muted>"Checkpoint lag: " {lag}</Body1>
                    {last_view}
                </Flex>
            </div>
        </A>
    }
}
