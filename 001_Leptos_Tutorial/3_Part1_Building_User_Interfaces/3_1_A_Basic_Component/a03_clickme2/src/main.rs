use leptos::{component, create_signal, view, IntoView};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count(3);
            }
        >
            "Click me: "
            {move || count()}
            </button>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
