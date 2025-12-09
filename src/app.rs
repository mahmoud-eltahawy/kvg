use cards::{Cards, CardsServerProps};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::app::xlsx_form::XlsxForm;

mod cards;
mod xlsx_form;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html dir="rtl" lang="en">
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
    let csp = RwSignal::new(None::<CardsServerProps>);
    let title = RwSignal::new(String::new());

    let xlsx_form = move || {
        view! {
            <XlsxForm title csp/>
        }
    };

    view! {
        <ShowLet some=csp let:csp fallback=xlsx_form>
            <Cards title=title.get() csp/>
        </ShowLet>
    }
}
