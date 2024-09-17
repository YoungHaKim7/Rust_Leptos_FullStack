use leptos::{component, view, IntoView};
use stylance::import_style;

import_style!(style, "main.module.scss");

#[component]
fn App() -> impl IntoView {
    view! {
        <p class=style::red>"test p scss"</p>
            <div class=style::one> one BackGround color Blue</div>
                <h1 id="two">"Hello"</h1>
                <h2>"World"</h2>
                <h2>"and"</h2>
                <h3 class="red">"friends!" </h3>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
