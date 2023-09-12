use common::game_state::GameStateSignal;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod atoms;
pub mod common;
pub mod error_template;
pub mod molecules;
pub mod organisms;
pub mod pages;
use crate::pages::{play::PlayPage,home::Home};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    log!("Setting up game state");
    provide_context(cx, create_rw_signal(cx, GameStateSignal::new(cx)));

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-actix-workspace.css"/>

        <meta name="viewport" content="width=device-width, initial-scale=1"/>
        // sets the document title
        <Title text="Welcome to Hive"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                    <Route path="/play" view=|cx| view! {cx, <PlayPage/>}/>
                </Routes>
            </main>
        </Router>
    }
}

