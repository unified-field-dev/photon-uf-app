use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Body1, Card, ContentContainer, EmptyState, SpacingSize, Title3};
use orbital::primitives::{Flex, FlexGap, Input, InputAppearance, MessageBar, MessageBarIntent};
use std::sync::OnceLock;

use crate::components::TopicCard;
use crate::server::{get_topics, TopicSummary};

static TOPICS_SPOTLIGHT_CARD: OnceLock<String> = OnceLock::new();

/// Topic index: searchable list of all registered topics with traffic summaries.
#[component]
pub fn PhotonTopicsIndexPage() -> impl IntoView {
    let _navigate = use_navigate();
    let search_query = RwSignal::new(String::new());
    let topics_res = Resource::new(|| (), |()| async move { get_topics().await });

    let filtered = Memo::new(move |_| {
        let topics: Vec<TopicSummary> = match topics_res.get() {
            Some(Ok(t)) => t,
            _ => return vec![],
        };
        let q = search_query.get().to_lowercase();
        let q = q.trim();
        if q.is_empty() {
            topics
        } else {
            topics
                .into_iter()
                .filter(|t| {
                    t.topic_name.to_lowercase().contains(q)
                        || t.schema_json.to_lowercase().contains(q)
                })
                .collect::<Vec<_>>()
        }
    });

    // Muted meta line; Orbital Body1 has no muted intent prop.
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Meta { color: var(--colorNeutralForeground3); }
        .SearchBox { max-width: 400px; }
    };

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-topics">
            <Flex vertical=true gap=SpacingSize::Size240.flex_gap()>
                <Title3>"Topics"</Title3>

                <Suspense fallback=move || view! { <Card>"Loading topics..."</Card> }>
                    {move || {
                        match topics_res.get() {
                        Some(Ok(_)) => {
                            let f = filtered.get();
                            let total = f.len();
                            let all_count = topics_res.get().map_or(0, |r| r.as_ref().ok().map_or(0, Vec::len));
                            view! {
                                <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
                                    <div class=class_names.search_box id="photon-topics-search">
                                        <Input bind=search_query appearance=InputAppearance::with_placeholder("Search topics...") />
                                    </div>
                                    <Card>
                                        {if f.is_empty() {
                                            let is_search = !search_query.get().trim().is_empty();
                                            let msg: &'static str = if is_search {
                                                "No topics match your search"
                                            } else {
                                                "No topics"
                                            };
                                            (if is_search {
                                                view! {
                                                    <EmptyState message=msg description="Try a different search term." />
                                                }
                                            } else {
                                                view! {
                                                    <EmptyState message=msg />
                                                }
                                            }).into_any()
                                        } else {
                                            view! {
                                                <Flex vertical=true gap=FlexGap::Medium>
                                                    <For
                                                        each=move || filtered.get()
                                                        key=|t| t.topic_name.clone()
                                                        let:t
                                                    >
                                                        {
                                                            let spotlight_ids = TOPICS_SPOTLIGHT_CARD
                                                                .set(t.topic_name.clone())
                                                                .is_ok();
                                                            view! {
                                                                <TopicCard topic=t spotlight_ids=spotlight_ids />
                                                            }
                                                        }
                                                    </For>
                                                </Flex>
                                            }.into_any()
                                        }}
                                    </Card>
                                    <Body1 block=true class=class_names.meta>
                                        "Showing " {total} " of " {all_count} " topics"
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
