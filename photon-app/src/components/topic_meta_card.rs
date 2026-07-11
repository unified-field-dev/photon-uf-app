//! Topic metadata card used on the topic detail page.

use leptos::prelude::*;
use orbital::components::Body1;
use orbital::primitives::*;

use crate::server::TopicSummary;
use orbital::components::Card;

/// Renders a card showing topic metadata: keyed-by, schema, and counts.
#[component]
pub fn TopicMetaCard(
    /// Topic data to display.
    topic: TopicSummary,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Meta { color: var(--colorNeutralForeground3); margin-bottom: 8px; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <Body1 block=true class=class_names.meta>"Keyed by: " {topic.keyed_by.clone().unwrap_or_else(|| "-".to_string())}</Body1>
            <Body1 block=true class=class_names.meta>"Schema: " {topic.schema_json.clone()}</Body1>
            <Body1 block=true class=class_names.meta>"Events (24h): " {topic.event_count_24h} " | Subscriptions: " {topic.subscription_count}</Body1>
        </Card>
    }
}
