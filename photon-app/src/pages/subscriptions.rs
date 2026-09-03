use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Body1, Card, ContentContainer, EmptyState, SpacingSize, Title3};
use orbital::primitives::{Flex, FlexGap, MessageBar, MessageBarIntent};
use std::sync::OnceLock;

use crate::components::{SubscriptionCard, SubscriptionFilterToolbar};
use crate::server::{get_subscriptions, SubscriptionSummary};

static SUBS_SPOTLIGHT_CARD: OnceLock<String> = OnceLock::new();

/// Subscription index: searchable, filterable list of all subscriptions.
#[component]
pub fn PhotonSubscriptionsIndexPage() -> impl IntoView {
    let _navigate = use_navigate();
    let search_query = RwSignal::new(String::new());
    let status_str = RwSignal::new(String::new());

    let subs_res = Resource::new(|| (), |()| async move { get_subscriptions().await });

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

    // Muted meta line; Orbital Body1 has no muted intent prop.
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Meta { color: var(--colorNeutralForeground3); }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-subscriptions">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Subscriptions"</Title3>

                <SubscriptionFilterToolbar search_query=search_query status_str=status_str />

                <Suspense fallback=move || view! { <Card>"Loading..."</Card> }>
                    {move || {
                        match subs_res.get() {
                        Some(Ok(_)) => {
                            let f = filtered.get();
                            let total = f.len();
                            let all_count = subs_res.get().map_or(0, |r| r.as_ref().ok().map_or(0, Vec::len));
                            view! {
                                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
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
                                                <Flex vertical=true gap=FlexGap::Medium>
                                                    <For
                                                        each=move || filtered.get()
                                                        key=|s| s.subscription_id.clone()
                                                        let:s
                                                    >
                                                        {
                                                            let spotlight_ids = SUBS_SPOTLIGHT_CARD
                                                                .set(s.subscription_id.clone())
                                                                .is_ok();
                                                            view! {
                                                                <SubscriptionCard sub=s spotlight_ids=spotlight_ids />
                                                            }
                                                        }
                                                    </For>
                                                </Flex>
                                            }.into_any()
                                        }}
                                    </Card>
                                    <Body1 block=true class=class_names.meta>
                                        "Showing " {total} " of " {all_count} " subscriptions"
                                    </Body1>
                                </Flex>
                            }.into_any()
                        }
                        Some(Err(e)) => view! { <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar> }.into_any(),
                        None => view! { <Card>"Loading..."</Card> }.into_any(),
                        }
                    }}
                </Suspense>
            </Flex>
        </ContentContainer>
    }
}
