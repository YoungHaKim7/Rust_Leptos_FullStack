use leptos::{component, create_signal, view, IntoView, ReadSignal, SignalUpdate};

#[component]
fn ProgressBar(progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max="50"
            // now this works
            value=progress
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
    <button
        on:click=move |_| {
            set_count.update(|n| *n += 1); } >
            "Click me ~~"
        </button>
        <ProgressBar progress=count/>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
