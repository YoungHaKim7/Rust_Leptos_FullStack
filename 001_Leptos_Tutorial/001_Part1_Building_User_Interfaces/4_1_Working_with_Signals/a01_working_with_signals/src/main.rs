use leptos::{component, create_signal, logging, IntoView, SignalGet, SignalSet};

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    set_count(1);
    logging::log!("{}", count());
    let (count02, set_count02) = create_signal(0);
    set_count02.set(1);
    logging::log!("{:?}", count.get());
}

fn main() {
    leptos::mount_to_body(App)
}
