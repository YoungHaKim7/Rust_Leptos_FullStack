use leptos::{component, use_context, view, IntoView, SignalUpdate, WriteSignal};

#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
            >
            "Toggle"
            </button>
    }
}
