#![allow(clippy::redundant_closure)]
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use orbital::components::{Card, ContentContainer, Title3};
use orbital::primitives::{MessageBar, MessageBarIntent};

use crate::components::EventMetaCard;
use crate::server::get_event;

/// Detail view for a single event: metadata, payload, and actor context.
#[component]
pub fn PhotonEventDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params.get().get("id").unwrap_or_default()
    };

    let event_res = Resource::new(move || id(), |eid| async move { get_event(eid).await });

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Header { margin-bottom: 24px; }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-event-detail">
            <div class=class_names.header>
                <Title3>"Event: " {move || id()}</Title3>
            </div>

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
        </ContentContainer>
    }
}
