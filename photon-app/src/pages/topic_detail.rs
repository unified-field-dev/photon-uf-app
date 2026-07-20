#![allow(clippy::redundant_closure)]
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use orbital::components::{Card, ContentContainer, Subtitle2, Title3};
use orbital::primitives::*;

use crate::components::{EventsTable, TopicMetaCard, TopicSubscriptionsTable};
use crate::server::{get_events, get_subscriptions, get_topic};

/// Detail view for a single topic: schema, subscriptions, and recent events.
#[component]
pub fn PhotonTopicDetailPage() -> impl IntoView {
    let params = use_params_map();
    let topic_name = move || {
        params
            .get()
            .get("topic_name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    };
    let _navigate = use_navigate();

    let topic_res = Resource::new(
        move || topic_name(),
        |name| async move { get_topic(name).await },
    );
    let events_res = Resource::new(
        move || topic_name(),
        |name| async move { get_events(Some(name), 20).await },
    );
    let subs_res = Resource::new(|| (), |_| async move { get_subscriptions().await });

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Header { margin-bottom: 24px; }
        .Section { margin-bottom: 24px; }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-topic-detail">
            <div class=class_names.header>
                <Title3>"Topic: " {move || topic_name()}</Title3>
            </div>

            <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                {move || match topic_res.get() {
                    Some(Ok(Some(t))) => view! {
                        <div class=class_names.section>
                            <TopicMetaCard topic=t />
                        </div>
                    }.into_any(),
                    Some(Ok(None)) => view! { <MessageBar intent=MessageBarIntent::Warning>"Topic not found"</MessageBar> }.into_any(),
                    Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                    None => view! { <Card>"Loading..."</Card> }.into_any(),
                }}
            </Suspense>

            <div class=class_names.section>
                <Subtitle2>"Subscriptions for this topic"</Subtitle2>
                <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                    {move || {
                        let name = topic_name();
                        match subs_res.get() {
                            Some(Ok(subs)) => {
                                let filtered: Vec<_> = subs.iter().filter(|s| s.topic_name == name).cloned().collect();
                                view! {
                                    <TopicSubscriptionsTable subs=filtered />
                                }.into_any()
                            }
                            _ => view! { <Card>"Loading..."</Card> }.into_any(),
                        }
                    }}
                </Suspense>
            </div>

            <div class=class_names.section>
                <Subtitle2>"Recent events"</Subtitle2>
                <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                    {move || {
                        match events_res.get() {
                            Some(Ok(events)) => view! {
                                <Card>
                                    <EventsTable
                                        events=events
                                        show_event_id=true
                                        show_topic=false
                                        show_key=false
                                        show_seq=true
                                        show_created=true
                                    />
                                </Card>
                            }.into_any(),
                            Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                            None => view! { <Card>"Loading..."</Card> }.into_any(),
                        }
                    }}
                </Suspense>
            </div>
        </ContentContainer>
    }
}
