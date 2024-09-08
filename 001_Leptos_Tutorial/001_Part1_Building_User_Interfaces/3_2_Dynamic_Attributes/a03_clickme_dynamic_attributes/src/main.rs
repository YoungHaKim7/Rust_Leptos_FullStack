// use leptos::{component, create_signal, view, IntoView, SignalUpdate};
//
// #[component]
// fn App() -> impl IntoView {
//     let (count, set_count) = create_signal(0);
//
//     view! {
//         <button
//             on:click=move |_| {
//                 set_count.update(|n| *n += 1);
//             }
//             class:red=move || count() % 2 == 1
//         >
//             "Click me: "
//             {move || count()}
//         </button>
//     }
// }
//
// fn main() {
//     leptos::mount_to_body(App)
// }
//
//
use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class={move || if count() % 2 == 1 { "red" } else{""}}
        >
            "Click me: "
            {move || count()}
        </button>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
