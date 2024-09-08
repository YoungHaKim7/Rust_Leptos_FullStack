use leptos::{component, create_signal, logging, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    let message = move || {
        if value() > 5 {
            logging::log!("{}: rendering Big", value());
            "Big"
        } else {
            logging::log!("{}: rendering Small", value());
            "Small"
        }
    };
    view! {
        <p>{message}</p>
    }
}
