use leptos::{component, create_signal, ev::MouseEvent, view, IntoView, SignalUpdate};

#[component]
pub fn ButtonB<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
