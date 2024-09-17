use leptos::{component, view, IntoView};
use stylance::import_style;

import_style!(style, "main.module.scss");

#[component]
fn App() -> impl IntoView {
    view! {
        <p class=style::one>"test p scss"</p>
            <div class="one-1ee4825"> one BackGround color Blue</div>
                <h1 id="two">"Hello"</h1>
                <h2 class="three-1ee4825">"World"</h2>
                <h3 class="red-1ee4825">"friends!" </h3>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
