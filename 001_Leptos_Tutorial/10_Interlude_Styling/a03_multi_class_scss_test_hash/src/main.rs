use leptos::{component, view, IntoView};
use stylance::import_style;

import_style!(style, "main.module.scss");

#[component]
fn App() -> impl IntoView {
    view! {
        <p class=style::one>"test p scss"</p>
            <div class="my-project-one-1ee4825"> one BackGround color Blue</div>
                <h1 id="two">"Hello"</h1>
                <h2>"World"</h2>
                <p class="my-project-three-1ee4825 my-project-red-1ee4825">"and"</p>
                <h3 class="my-project-red-1ee4825">"friends!" </h3>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
