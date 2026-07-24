//! Active subscriptions table for the Photon dashboard.

use leptos::prelude::*;

use crate::components::SubscriptionStatusBadge;
use crate::server::SubscriptionSummary;
use orbital::primitives::{Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow};

/// Renders a table of subscriptions with Name, Topic, Status, and Lag columns.
#[component]
pub fn ActiveSubscriptionsTable(
    /// Subscription rows to display.
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
        <Table class=class_names.table>
            <TableHeader>
                <TableRow>
                    <TableHeaderCell>"Name"</TableHeaderCell>
                    <TableHeaderCell>"Topic"</TableHeaderCell>
                    <TableHeaderCell>"Status"</TableHeaderCell>
                    <TableHeaderCell>"Lag"</TableHeaderCell>
                </TableRow>
            </TableHeader>
            <TableBody>
                <For
                    each=move || subs.get_value()
                    key=|s| s.subscription_id.clone()
                    let:s
                >
                    <TableRow class=class_names.row>
                        <TableCell><a href=crate::paths::subscription(&s.subscription_id) class=class_names.link>{s.subscription_name}</a></TableCell>
                        <TableCell>{s.topic_name}</TableCell>
                        <TableCell><SubscriptionStatusBadge enabled=s.enabled /></TableCell>
                        <TableCell>{s.checkpoint_lag}</TableCell>
                    </TableRow>
                </For>
            </TableBody>
        </Table>
    }
}
