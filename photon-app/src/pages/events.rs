use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Body1, Card, ContentContainer, EmptyState, SpacingSize, Title3};
use orbital::primitives::{Flex, MessageBar, MessageBarIntent};

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

    let topics_res = Resource::new(|| (), |()| async move { get_topics().await });
    let events_res = Resource::new(
        move || topic_filter.get(),
        |topic_opt| async move { get_events(topic_opt, 100).await },
    );

    // Muted meta line; Orbital Body1 has no muted intent prop.
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Meta { color: var(--colorNeutralForeground3); }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-events">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Events"</Title3>

                <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                    {move || match events_res.get() {
                        Some(Ok(events)) => {
                            let total = events.len();
                            let topics: Vec<_> = topics_res.get().and_then(Result::ok).unwrap_or_default();
                            view! {
                                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
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
                                    <Body1 block=true class=class_names.meta>
                                        "Showing " {total} " events"
                                    </Body1>
                                </Flex>
                            }.into_any()
                        }
                        Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                        None => view! { <Card>"Loading..."</Card> }.into_any(),
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}
