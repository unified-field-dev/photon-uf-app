#![allow(clippy::redundant_closure)]
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};
use orbital::components::{Card, ContentContainer, Subtitle2, Title3};
use orbital::primitives::{MessageBar, MessageBarIntent};

use crate::components::{EventsTable, SubscriptionMetaCard};
use crate::server::{get_events, get_subscription};

/// Detail view for a single subscription: configuration, read-state, and recent events.
#[component]
pub fn PhotonSubscriptionDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").unwrap_or_default();
    let _navigate = use_navigate();

    let sub_res = Resource::new(
        move || id(),
        |sid| async move { get_subscription(sid).await },
    );
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

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Header { margin-bottom: 24px; }
        .Section { margin-bottom: 24px; }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-subscription-detail">
            <div class=class_names.header>
                <Title3>"Subscription: " {move || id()}</Title3>
            </div>

            <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                {move || match sub_res.get() {
                    Some(Ok(Some(s))) => view! {
                        <div class=class_names.section>
                            <SubscriptionMetaCard sub=s />
                        </div>
                    }.into_any(),
                    Some(Ok(None)) => view! { <MessageBar intent=MessageBarIntent::Warning>"Subscription not found"</MessageBar> }.into_any(),
                    Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                    None => view! { <Card>"Loading..."</Card> }.into_any(),
                }}
            </Suspense>

            <div class=class_names.section>
                <Subtitle2>"Recent events (topic)"</Subtitle2>
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
