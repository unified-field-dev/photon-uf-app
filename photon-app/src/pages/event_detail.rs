use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Card, ContentContainer, SpacingSize, Title3};
use orbital::primitives::{Flex, MessageBar, MessageBarIntent};

use crate::components::EventMetaCard;
use crate::server::get_event;

/// Detail view for a single event: metadata, payload, and actor context.
#[component]
pub fn PhotonEventDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get().get("id").unwrap_or_default();

    let event_res = Resource::new(id.clone(), |eid| async move { get_event(eid).await });

    view! {
        <ContentContainer data_testid="photon-event-detail">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Event: " {move || id()}</Title3>

                <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                    {move || match event_res.get() {
                        Some(Ok(Some(ev))) => view! {
                            <EventMetaCard event=ev />
                        }.into_any(),
                        Some(Ok(None)) => view! { <MessageBar intent=MessageBarIntent::Warning>"Event not found"</MessageBar> }.into_any(),
                        Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                        None => view! { <Card>"Loading..."</Card> }.into_any(),
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}
