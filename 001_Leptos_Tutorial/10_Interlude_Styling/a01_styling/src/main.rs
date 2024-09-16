use leptos::{component, view, IntoView};
use stylance::import_style;

import_style!(style, "component/card/card.module.scss");

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class=style::jumbotron/>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
    class = styler_class,
            <div class="one">
                <h1 id="two">"Hello"</h1>
                <h2>"World"</h2>
                <h2>"and"</h2>
                <h3>"friends!" HomePage</h3>    }
}

fn main() {
    leptos::mount_to_body(App)
}
