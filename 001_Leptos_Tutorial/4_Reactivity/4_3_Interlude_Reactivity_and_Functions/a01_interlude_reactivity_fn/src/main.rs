use leptos::{component, create_effect, create_signal, logging, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    // a signal holds a value, and can be updated
    let (count, set_count) = create_signal(0);

    // a derived signal is a function that accesses other signals
    let double_count = move || count() * 2;
    let count_is_odd = move || count() & 1 == 1;
    let text = move || if count_is_odd() { "odd" } else { "even" };

    // an effect automatically tracks the signals it depends on
    // and reruns when they change
    create_effect(move |_| {
        logging::log!("text = {}", text());
    });

    view! {
        <p>{move || text().to_uppercase()}</p>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
