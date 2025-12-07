use std::{num::NonZero, path::PathBuf};

use cards::{Cards, CardsServerProps};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod cards;

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

#[component]
fn HomePage() -> impl IntoView {
    let title = "كارت ضاحية".to_string();

    let csp = CardsServerProps {
        title_row_index: NonZero::new(1),
        path: PathBuf::from("demo_excel.xlsx"),
        sheet: String::from("Sheet1"),
        columns_indexes: vec![0, 2, 3, 5],
    };

    view! {
        <Cards title csp/>
    }
}
