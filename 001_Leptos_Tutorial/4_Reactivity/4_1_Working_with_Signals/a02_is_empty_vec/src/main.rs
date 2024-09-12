use leptos::{
    component, create_effect, create_signal,
    logging::{self, log},
    view, IntoView, SignalGet, SignalUpdate, SignalWith,
};

#[component]
pub fn App() -> impl IntoView {
    let (names, set_names) = create_signal(Vec::new());
    if names.with(|names| names.is_empty()) {
        set_names.update(|names| names.push("Alice".to_string()));
    }
    logging::log!("{:?}", names.get());

    let (first, _) = create_signal("Bob".to_string());
    let (middle, _) = create_signal("J.".to_string());
    let (last, _) = create_signal("Smith".to_string());

    let names01 = move || {
        first.with(|first| {
            middle.with(|middle| last.with(|last| format!("{first} {middle} {last}")));
        })
    };

    logging::log!("{:?}", names01());
    // create_effect(move |_| {
    //     // immediately prints "Value: 0" and subscribes to `a`
    //     logging::log!("Value: {:?}", name01());
    // });
    view! {
        <h1>Hello</h1>
        <h1>{{names01}}</h1>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
