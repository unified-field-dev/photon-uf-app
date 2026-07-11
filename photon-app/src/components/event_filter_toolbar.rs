//! Topic filter toolbar for the events index page.

use leptos::prelude::*;

use crate::server::TopicSummary;
use orbital::primitives::*;

/// Renders a `Select` dropdown to filter events by topic.
#[component]
pub fn EventFilterToolbar(
    /// Two-way signal for the selected topic string.
    topic_str: RwSignal<String>,
    /// Available topics for the dropdown.
    topics: Vec<TopicSummary>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Toolbar { margin-bottom: 16px; }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.toolbar>
            <Select bind=topic_str>
                <option value="">"Topic: All"</option>
                {topics.iter().map(|t| {
                    let name = t.topic_name.clone();
                    view! { <option value=name.clone()>{name.clone()}</option> }
                }).collect_view()}
            </Select>
        </div>
    }
}
