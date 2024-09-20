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
        <figure class="my-project-figure-82fb418">
          <img class="w-24 h-24 md:w-48 md:h-auto md:rounded-none rounded-full mx-auto" src="imgs/test.jpg" alt="" width="384" height="512" />
          <div class="pt-6 md:p-8 text-center space-y-4">
            <blockquote>
              <p class="font-medium text-sky-500 dark:text-sky-400">
                "Tailwind CSS is the only framework that I've seen scale on large teams. It’s easy to customize, adapts to any design, and the build size is tiny."
              </p>
            </blockquote>
            <figcaption class="font-medium">
              <div class="text-sky-500 dark:text-sky-400">
                Gyoung
              </div>
              <div class="text-slate-700 dark:text-slate-500">
                Youtuber
              </div>
            </figcaption>
          </div>
        </figure>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
