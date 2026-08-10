//! Reusable events table with configurable columns.
//!
//! Used across the events index, topic detail, subscription detail, and
//! dashboard pages. Column visibility is controlled via [`EventsTableColumns`].

use leptos::prelude::*;

use crate::server::EventSummary;
use orbital::primitives::{Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow};

/// Which columns [`EventsTable`] renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventsTableColumns {
    /// Show the Event ID column (linked to detail page).
    pub show_event_id: bool,
    /// Show the Topic column.
    pub show_topic: bool,
    /// Show the Key column.
    pub show_key: bool,
    /// Show the Seq column.
    pub show_seq: bool,
    /// Show the Created column.
    pub show_created: bool,
}

impl Default for EventsTableColumns {
    fn default() -> Self {
        Self {
            show_event_id: true,
            show_topic: true,
            show_key: true,
            show_seq: true,
            show_created: true,
        }
    }
}

/// Reusable events table.
///
/// Renders a `Table` of events with an optional Event ID link column.
#[component]
pub fn EventsTable(
    /// Event rows to display.
    events: Vec<EventSummary>,
    /// Column visibility. Defaults to all columns shown.
    #[prop(optional)]
    columns: Option<EventsTableColumns>,
) -> impl IntoView {
    let columns = columns.unwrap_or_default();
    let events = StoredValue::new(events);
    let show_event_id = columns.show_event_id;
    let show_topic = columns.show_topic;
    let show_key = columns.show_key;
    let show_seq = columns.show_seq;
    let show_created = columns.show_created;

    // Row hover + brand link color; Orbital Table has no row-hover / cell-link props yet.
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

    view! {
        <style>{style_sheet}</style>
        <Table class=class_names.table>
            <TableHeader>
                <TableRow>
                    {show_event_id.then(|| view! { <TableHeaderCell>"Event ID"</TableHeaderCell> })}
                    {show_topic.then(|| view! { <TableHeaderCell>"Topic"</TableHeaderCell> })}
                    {show_key.then(|| view! { <TableHeaderCell>"Key"</TableHeaderCell> })}
                    {show_seq.then(|| view! { <TableHeaderCell>"Seq"</TableHeaderCell> })}
                    {show_created.then(|| view! { <TableHeaderCell>"Created"</TableHeaderCell> })}
                </TableRow>
            </TableHeader>
            <TableBody>
                <For
                    each=move || events.get_value()
                    key=|e| e.event_id.clone()
                    let:e
                >
                    {
                        let id = e.event_id.clone();
                        let href = crate::paths::event(&id);
                        let href2 = href.clone();
                        let topic = e.topic_name.clone();
                        let key = e.topic_key.clone().unwrap_or_default();
                        let created = e.created_at.clone();
                        view! {
                            <TableRow class=class_names.row>
                                {show_event_id.then(|| view! { <TableCell><a href=href class=class_names.link>{id}</a></TableCell> })}
                                {show_topic.then(|| {
                                    if show_event_id {
                                        view! { <TableCell>{topic}</TableCell> }
                                    } else {
                                        view! { <TableCell><a href=href2.clone() class=class_names.link>{topic}</a></TableCell> }
                                    }
                                })}
                                {show_key.then(|| view! { <TableCell>{key}</TableCell> })}
                                {show_seq.then(|| view! { <TableCell>{e.seq}</TableCell> })}
                                {show_created.then(|| view! { <TableCell>{created}</TableCell> })}
                            </TableRow>
                        }
                    }
                </For>
            </TableBody>
        </Table>
    }
}
