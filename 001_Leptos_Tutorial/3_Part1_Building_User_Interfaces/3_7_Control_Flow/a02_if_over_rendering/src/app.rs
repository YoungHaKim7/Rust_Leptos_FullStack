use leptos::{component, create_signal, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    let message = move || if value() > 5 { "Big" } else { "Small" };

    view! {
        <p>{message}</p>
    }
}
