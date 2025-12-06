use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kvg.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Card {
    id: usize,
    kv: Vec<Kv>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Kv {
    key: String,
    value: String,
}

#[server]
async fn get_cards() -> Result<Vec<Card>, ServerFnError> {
    let mut cards = Vec::new();
    for i in 0..100 {
        let mut kvs = Vec::new();
        for j in 0..7 {
            kvs.push(Kv {
                key: format!("key {j}"),
                value: format!("value {j}"),
            })
        }
        cards.push(Card { id: i, kv: kvs })
    }
    Ok(cards)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let cards = LocalResource::new(get_cards);

    let cardsfn = move || cards.get().transpose().ok().flatten().unwrap_or_default();

    view! {
        <div class="grid grid-cols-4 gap-5">
            <For
                each=cardsfn
                key=|x| x.id
                let(card)
                >
                    <div class="border-2">
                        <h2>"كارت ضاحية"</h2>
                        <ul>
                            <For
                                each=move || card.kv.clone()
                                key=|x| x.key.clone()
                                let(kv)
                            >
                                <li>
                                    <span class="w-min">{kv.key}</span> : <span class="w-min">{kv.value}</span>
                                </li>
                            </For>
                        </ul>
                    </div>
            </For>
        </div>
    }
}
