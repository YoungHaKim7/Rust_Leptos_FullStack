use leptos::{component, create_signal, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 2 == 2;
    // let message = move || {
    //     if is_even() {
    //         Some("Ding ding ding!")
    //     } else {
    //         None
    //     }
    // };
    let message = move || match value() {
        0 => "Zero",
        1 => "One",
        n if is_odd() => "Odd",
        _ => "Even",
    };
    view! {
        // <p>
        // {move || if is_even() {
        //     "Odd"
        // } else {
        //     "Even"
        // }}
        // </p>
        <p>{message}</p>
    }
}
