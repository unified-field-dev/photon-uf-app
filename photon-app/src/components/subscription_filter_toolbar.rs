//! Filter toolbar for the subscriptions index page.

use leptos::prelude::*;
use orbital::primitives::*;

/// Renders a search input and status select for filtering subscriptions.
#[component]
pub fn SubscriptionFilterToolbar(
    /// Two-way signal for the search text.
    search_query: RwSignal<String>,
    /// Two-way signal for the status filter value ("", "on", "off").
    status_str: RwSignal<String>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Toolbar {
            display: flex;
            gap: 12px;
            margin-bottom: 16px;
            flex-wrap: wrap;
            align-items: center;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.toolbar>
            <Input bind=search_query appearance=InputAppearance::with_placeholder("Search subscriptions...") />
            <Select bind=status_str>
                <option value="">"Status: All"</option>
                <option value="on">"ON"</option>
                <option value="off">"OFF"</option>
            </Select>
        </div>
    }
}
