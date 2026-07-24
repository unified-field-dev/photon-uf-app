//! Subscriptions table filtered to a specific topic.

use leptos::prelude::*;

use crate::server::SubscriptionSummary;
use orbital::components::Card;
use orbital::primitives::{Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow};

/// Renders a Name / Enabled table of subscriptions for a given topic.
#[component]
pub fn TopicSubscriptionsTable(
    /// Subscriptions to display (already filtered to the target topic).
    subs: Vec<SubscriptionSummary>,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Table { width: 100%; }
        .Row { cursor: pointer; }
        .Row:hover { background: var(--orb-color-surface-canvas-hover); }
        .Link {
            color: var(--colorBrandForeground1);
            text-decoration: none;
        }
        .Link:hover { text-decoration: underline; }
    };

    let subs = StoredValue::new(subs);

    view! {
        <style>{style_sheet}</style>
        <Card>
            <Table class=class_names.table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Enabled"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <For
                        each=move || subs.get_value()
                        key=|s| s.subscription_id.clone()
                        let:s
                    >
                        <TableRow class=class_names.row>
                            <TableCell><a href=crate::paths::subscription(&s.subscription_id) class=class_names.link>{s.subscription_name.clone()}</a></TableCell>
                            <TableCell>{if s.enabled { "Yes" } else { "No" }}</TableCell>
                        </TableRow>
                    </For>
                </TableBody>
            </Table>
        </Card>
    }
}
