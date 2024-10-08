use leptos::{
    component, create_signal, logging, mount_to_body, view, IntoView, SignalSet, SignalUpdate,
};

#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = create_signal(initial_value);
    logging::log!("{:?}", set_value);
    logging::log!("{:?}", value);

    view! {
        <div>
            <button on:click=move |_| set_value.set(0)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <SimpleCounter initial_value=0 step=1/>})
}
