use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Card, Body1, ContentContainer, EmptyState, Title3};
use orbital::primitives::*;

use crate::components::{EventFilterToolbar, EventsTable};
use crate::server::{get_events, get_topics};

/// Event index: filterable, paginated list of events across all topics.
#[component]
pub fn PhotonEventsIndexPage() -> impl IntoView {
    let _navigate = use_navigate();
    let topic_filter = RwSignal::new(Option::<String>::None);
    let topic_str = RwSignal::new(String::new());

    // Sync select string -> Option<String> filter
    Effect::new(move || {
        let val = topic_str.get();
        topic_filter.set(if val.is_empty() { None } else { Some(val) });
    });

    let topics_res = Resource::new(|| (), |_| async move { get_topics().await });
    let events_res = Resource::new(
        move || topic_filter.get(),
        |topic_opt| async move { get_events(topic_opt, 100).await },
    );

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Header { margin-bottom: 24px; }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-events">
            <div class=class_names.header>
                <Title3>"Events"</Title3>
            </div>

            <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                {move || match events_res.get() {
                    Some(Ok(events)) => {
                        let total = events.len();
                        let topics: Vec<_> = topics_res.get().and_then(|r| r.ok()).unwrap_or_default();
                        view! {
                            <EventFilterToolbar topic_str=topic_str topics=topics />
                            <Card>
                                {if events.is_empty() {
                                    view! {
                                        <EmptyState message="No events" />
                                    }.into_any()
                                } else {
                                    view! {
                                        <EventsTable events=events />
                                    }.into_any()
                                }}
                            </Card>
                            <Body1 block=true style="margin-top: 16px; color: var(--colorNeutralForeground3);">
                                "Showing " {total} " events"
                            </Body1>
                        }.into_any()
                    }
                    Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                    None => view! { <Card>"Loading..."</Card> }.into_any(),
                }}
            </Suspense>
        </ContentContainer>
    }
}
