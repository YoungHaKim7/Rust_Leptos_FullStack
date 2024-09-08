use leptos::{component, create_signal, view, IntoView, SignalUpdate, WriteSignal};

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonA setter=set_toggled/>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
