//! Event metadata card for the event detail page.

use leptos::prelude::*;
use orbital::components::{Card, Body1, Subtitle2, Text, TextTag, TextFont, TextSize};
use orbital::primitives::{MessageBar, MessageBarIntent};

use crate::server::EventDetail;

/// Renders event metadata (topic, key, seq, created) and JSON payload
/// and actor sections.
#[component]
pub fn EventMetaCard(
    /// Event data to display.
    event: EventDetail,
) -> impl IntoView {
    let transport_expired = event.transport_expired;
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Section { margin-bottom: 24px; }
        .Muted { color: var(--colorNeutralForeground3); }
        .CodeBlock {
            white-space: pre-wrap;
            word-break: break-all;
            padding: 12px;
            background: var(--colorNeutralBackground2);
            border-radius: 8px;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.section>
            <Card>
                <Body1 block=true class=class_names.muted>"Topic: " {event.topic_name.clone()}</Body1>
                <Body1 block=true class=class_names.muted>"Key: " {event.topic_key.clone().unwrap_or_else(|| "\u{2014}".to_string())}</Body1>
                <Body1 block=true class=class_names.muted>"Seq: " {event.seq}</Body1>
                <Body1 block=true class=class_names.muted>"Created: " {event.created_at.clone()}</Body1>
                <Body1 block=true class=class_names.muted>"Status: " {event.delivery_status.clone()}</Body1>
            </Card>
        </div>
        {move || transport_expired.then(|| view! {
            <MessageBar intent=MessageBarIntent::Warning>
                "Transport payload expired (retention). Metadata only."
            </MessageBar>
        })}
        <div class=class_names.section>
            <Subtitle2>"Payload"</Subtitle2>
            <Text tag=TextTag::Pre font=TextFont::Monospace size=TextSize::S200 block=true class=class_names.code_block>
                {serde_json::to_string_pretty(&event.payload_json).unwrap_or_default()}
            </Text>
        </div>
        <div class=class_names.section>
            <Subtitle2>"Actor"</Subtitle2>
            <Text tag=TextTag::Pre font=TextFont::Monospace size=TextSize::S200 block=true class=class_names.code_block>
                {serde_json::to_string_pretty(&event.actor_json).unwrap_or_default()}
            </Text>
        </div>
    }
}
