use leptos::{component, view, IntoView};
use stylance::import_style;

import_style!(style, "./scss/main.module.scss");

#[component]
fn App() -> impl IntoView {
    view! {
        <div class=style::container_body>
            <p class=style::one>"test p scss"</p>
                <div class="my-project-one-c7311a6"> one BackGround color Blue</div>
                    <h1 id="two">"Hello"</h1>
                    <h2 class="my-project-three-c7311a6">"World"</h2>
                    <h3 class="my-project-red-c7311a6">"friends!" </h3>
                    <h3 class=style::red>"red font!" </h3>
                    <h2 class="my-project-three-c7311a6 my-project-red-c7311a6">"class class test" </h2>
                    <h2 class=style::three>"three three" </h2>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
