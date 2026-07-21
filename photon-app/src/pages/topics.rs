use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{Body1, Card, ContentContainer, EmptyState, Title3};
use orbital::primitives::{Input, InputAppearance, MessageBar, MessageBarIntent};

use crate::components::TopicCard;
use crate::server::{get_topics, TopicSummary};

/// Topic index: searchable list of all registered topics with traffic summaries.
#[component]
pub fn PhotonTopicsIndexPage() -> impl IntoView {
    let _navigate = use_navigate();
    let search_query = RwSignal::new(String::new());
    let topics_res = Resource::new(|| (), |()| async move { get_topics().await });

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Header { margin-bottom: 24px; }
        .SearchBox { margin-bottom: 16px; max-width: 400px; }
        .CardGrid { display: flex; flex-direction: column; gap: 16px; }
        .Meta { color: var(--colorNeutralForeground3); margin-bottom: 4px; }
    };

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

    view! {
        <style>{style_sheet}</style>
        <ContentContainer data_testid="photon-topics">
            <div class=class_names.header>
                <Title3>"Topics"</Title3>
            </div>

            <Suspense fallback=move || view! { <Card>"Loading topics..."</Card> }>
                {move || {
                    match topics_res.get() {
                    Some(Ok(_)) => {
                        let f = filtered.get();
                        let total = f.len();
                        let all_count = topics_res.get().map_or(0, |r| r.as_ref().ok().map_or(0, Vec::len));
                        view! {
                            <div class=class_names.search_box>
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
                                        <div class=class_names.card_grid>
                                            <For
                                                each=move || filtered.get()
                                                key=|t| t.topic_name.clone()
                                                let:t
                                            >
                                                <TopicCard topic=t />
                                            </For>
                                        </div>
                                    }.into_any()
                                }}
                            </Card>
                            <Body1 block=true class=class_names.meta style="margin-top: 16px;">
                                "Showing " {total} " of " {all_count} " topics"
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
