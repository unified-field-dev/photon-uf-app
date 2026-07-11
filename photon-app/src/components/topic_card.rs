//! Topic card used in the topics index page.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Body1, Body1Strong, SpacingSize};
use orbital::primitives::*;

use crate::server::TopicSummary;

/// Renders a single topic card with name, keyed-by, schema, event/subscription
/// counts, and action buttons.
#[component]
pub fn TopicCard(
    /// Topic data to display.
    topic: TopicSummary,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .TopicCard {
            padding: 16px;
            border: 1px solid var(--colorNeutralStroke1);
            border-radius: 8px;
            cursor: pointer;
            background: var(--colorNeutralBackground1);
        }
        .TopicCard:hover { background: var(--colorNeutralBackground1Hover); }
        .Muted { color: var(--colorNeutralForeground3); }
        .Actions { margin-top: 12px; display: flex; gap: 8px; }
    };

    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let name = topic.topic_name.clone();
    let keyed = topic.keyed_by.is_some();
    let test_id = format!("topic-{}", &name);
    let name_click = name.clone();
    let name_btn = name.clone();
    let keyed_by_view = topic.keyed_by.clone().map(|k| {
        view! {
            <Body1 block=true class=class_names.muted>"Keyed by: " {k}</Body1>
        }
    });
    let schema = topic.schema_json.clone();
    let event_count = topic.event_count_24h;
    let sub_count = topic.subscription_count;

    view! {
        <style>{style_sheet}</style>
        <div
            class=class_names.topic_card
            data-testid=test_id
            on:click={
                let nav = nav_store.with_value(|n: &_| n.clone());
                let name_click = name_click.clone();
                move |_| nav(&crate::paths::topic(&name_click), Default::default())
            }
        >
            <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                <Body1Strong block=true>{name.clone()}{if keyed { " KEYED" } else { "" }}</Body1Strong>
                {keyed_by_view}
                <Body1 block=true class=class_names.muted>"Schema: " {schema}</Body1>
                <Body1 block=true class=class_names.muted>"Events (24h): " {event_count} " | Subscriptions: " {sub_count}</Body1>
            </Flex>
            <div class=class_names.actions>
                <Button size=ButtonSize::Small appearance=ButtonAppearance::Subtle on_click=Callback::new({
                    let nav = nav_store.with_value(|n: &_| n.clone());
                    let name_btn = name_btn.clone();
                    move |_| nav(&crate::paths::topic(&name_btn), Default::default())
                })>"View"</Button>
                <Button size=ButtonSize::Small appearance=ButtonAppearance::Subtle on_click=Callback::new({
                    let nav = nav_store.with_value(|n: &_| n.clone());
                    move |_| nav(crate::paths::EVENTS, Default::default())
                })>"View Events"</Button>
                <Button size=ButtonSize::Small appearance=ButtonAppearance::Subtle on_click=Callback::new({
                    let nav = nav_store.with_value(|n: &_| n.clone());
                    move |_| nav(crate::paths::SUBSCRIPTIONS, Default::default())
                })>"View Subscriptions"</Button>
            </div>
        </div>
    }
}
