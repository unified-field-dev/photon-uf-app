use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Card, Body1, ContentContainer, EmptyState, Title3};
use orbital::primitives::*;

use crate::components::{SubscriptionCard, SubscriptionFilterToolbar};
use crate::server::{get_subscriptions, SubscriptionSummary};

#[component]
pub fn PhotonSubscriptionsIndexPage() -> impl IntoView {
    let _navigate = use_navigate();
    let search_query = RwSignal::new(String::new());
    let status_str = RwSignal::new(String::new());

    let subs_res = Resource::new(|| (), |_| async move { get_subscriptions().await });

    let filtered = Memo::new(move |_| {
        let subs: Vec<SubscriptionSummary> = match subs_res.get() {
            Some(Ok(s)) => s,
            _ => return vec![],
        };
        let q = search_query.get().to_lowercase().trim().to_string();
        let status: Option<bool> = match status_str.get().as_str() {
            "on" => Some(true),
            "off" => Some(false),
            _ => None,
        };
        subs.into_iter()
            .filter(|s| {
                let matches_search = q.is_empty()
                    || s.subscription_name.to_lowercase().contains(&q)
                    || s.topic_name.to_lowercase().contains(&q);
                let matches_status = match status {
                    None => true,
                    Some(true) => s.enabled,
                    Some(false) => !s.enabled,
                };
                matches_search && matches_status
            })
            .collect::<Vec<_>>()
    });

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Header { margin-bottom: 24px; }
        .CardGrid { display: flex; flex-direction: column; gap: 16px; }
        .Meta { color: var(--colorNeutralForeground3); margin-bottom: 4px; }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-subscriptions">
            <div class=class_names.header>
                <Title3>"Subscriptions"</Title3>
            </div>

            <SubscriptionFilterToolbar search_query=search_query status_str=status_str />

            <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                {move || {
                    match subs_res.get() {
                    Some(Ok(_)) => {
                        let f = filtered.get();
                        let total = f.len();
                        let all_count = subs_res.get().map(|r| r.as_ref().ok().map(|v| v.len()).unwrap_or(0)).unwrap_or(0);
                        view! {
                            <Card>
                                {if f.is_empty() {
                                    let msg: &'static str = if search_query.get_untracked().trim().is_empty() {
                                        "No subscriptions"
                                    } else {
                                        "No subscriptions match your filters"
                                    };
                                    view! {
                                        <EmptyState message=msg />
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class=class_names.card_grid>
                                            <For
                                                each=move || filtered.get()
                                                key=|s| s.subscription_id.clone()
                                                let:s
                                            >
                                                <SubscriptionCard sub=s />
                                            </For>
                                        </div>
                                    }.into_any()
                                }}
                            </Card>
                            <Body1 block=true class=class_names.meta style="margin-top: 16px;">
                                "Showing " {total} " of " {all_count} " subscriptions"
                            </Body1>
                        }.into_any()
                    }
                    Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                    None => view! { <Card>"Loading..."</Card> }.into_any(),
                    }
                }}
            </Suspense>
        </ContentContainer>
    }
}
