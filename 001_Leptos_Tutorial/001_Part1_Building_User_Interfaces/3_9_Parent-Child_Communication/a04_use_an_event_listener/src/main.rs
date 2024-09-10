use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>
            "Toggle"
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonC on:click=move |_| set_toggled.update(|value| *value =!*value)/>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
