use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod db;
pub mod models;
pub mod pages;
pub mod server_functions;
use pages::{HomePage, TeamPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/actix-app01.css"/>
        <link data-trunk rel="tailwind-css" href="/style/input.css"/>

        // sets the document title
        <Title text="Full-Stack Dashboard App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <HomePage />
                        }
                    }/>
                    <Route path="/team" view=move || {
                        view! {
                            <TeamPage />
                        }
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
