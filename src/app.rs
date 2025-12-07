use std::path::PathBuf;

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
            <body dir="rtl">
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
        <Title text="kvg"/>

        // content for this welcome page
        <Router>
            <main class="m-4">
                <p class="text-xs text-left p-3 print:hidden">made by mahmoud eltahawy</p>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Card {
    row_index: usize,
    kv: Vec<Kv>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Kv {
    key: String,
    value: String,
}

#[server]
async fn get_cards(
    path: PathBuf,
    sheet: String,
    indexs: Vec<usize>,
) -> Result<Vec<Card>, ServerFnError> {
    use calamine::{open_workbook, Data, DeError, RangeDeserializerBuilder, Reader, Xlsx};

    let mut workbook: Xlsx<_> = open_workbook(&path)?;
    let range = workbook.worksheet_range(&sheet)?;

    let mut iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    let headers = iter
        .next()
        .unwrap_or(Err(DeError::HeaderNotFound(String::from(
            "Error : first row should contain headers",
        ))))?;

    let mut cards = Vec::new();
    for (i, row) in iter.enumerate() {
        let mut kvs = Vec::new();
        let row: Vec<Data> = row?;
        for index in indexs.iter() {
            let header = headers[*index].to_string();
            let value = row[*index].to_string();
            if !header.is_empty() && !value.is_empty() {
                kvs.push(Kv {
                    key: header,
                    value: value,
                });
            }
        }
        cards.push(Card {
            row_index: i,
            kv: kvs,
        });
    }

    Ok(cards)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let cards = LocalResource::new(|| {
        get_cards(
            PathBuf::from("demo_excel.xlsx"),
            String::from("Sheet1"),
            vec![2, 3, 5],
        )
    });

    let cardsfn = move || cards.get().transpose().ok().flatten().unwrap_or_default();

    view! {
        <div class="grid grid-cols-4 gap-5">
            <For
                each=cardsfn
                key=|x| x.row_index
                let(Card { row_index:_, kv })
                >
                    <div class="border-sky-500 border-5 rounded-xl p-2 text-3xl text-center">
                        <h2 class="font-bold">"كارت ضاحية"</h2>
                        <ul>
                            <For
                                each=move || kv.clone()
                                key=|x| x.key.clone()
                                let(Kv { key, value })
                            >
                                <li>
                                    {format!("{key} : {value}")}
                                </li>
                            </For>
                        </ul>
                    </div>
            </For>
        </div>
    }
}
