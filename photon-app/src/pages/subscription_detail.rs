use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Card, ContentContainer, SpacingSize, Subtitle2, Title3};
use orbital::primitives::{Flex, MessageBar, MessageBarIntent};

use crate::components::{EventsTable, EventsTableColumns, SubscriptionMetaCard};
use crate::server::{get_events, get_subscription};

/// Detail view for a single subscription: configuration, read-state, and recent events.
#[component]
pub fn PhotonSubscriptionDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").unwrap_or_default();

    let sub_res = Resource::new(id, |sid| async move { get_subscription(sid).await });
    let topic_for_events = move || {
        sub_res
            .get()
            .and_then(|r| r.ok().flatten())
            .map(|s| s.topic_name)
    };
    let events_res = Resource::new(topic_for_events, |topic_opt| async move {
        if let Some(topic) = topic_opt {
            get_events(Some(topic), 20).await
        } else {
            Ok(vec![])
        }
    });

    view! {
        <ContentContainer data_testid="photon-subscription-detail">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Subscription: " {move || id()}</Title3>

                <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                    {move || match sub_res.get() {
                        Some(Ok(Some(s))) => view! {
                            <SubscriptionMetaCard sub=s />
                        }.into_any(),
                        Some(Ok(None)) => view! { <MessageBar intent=MessageBarIntent::Warning>"Subscription not found"</MessageBar> }.into_any(),
                        Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                        None => view! { <Card>"Loading..."</Card> }.into_any(),
                    }}
                </Suspense>

                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                    <Subtitle2>"Recent events (topic)"</Subtitle2>
                    <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                        {move || {
                            match events_res.get() {
                            Some(Ok(events)) => view! {
                                <Card>
                                    <EventsTable
                                        events=events
                                        columns=EventsTableColumns {
                                            show_event_id: true,
                                            show_topic: false,
                                            show_key: false,
                                            show_seq: true,
                                            show_created: true,
                                        }
                                    />
                                </Card>
                            }.into_any(),
                            Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                            None => view! { <Card>"Loading..."</Card> }.into_any(),
                            }
                        }}
                    </Suspense>
                </Flex>
            </Flex>
        </ContentContainer>
    }
}
