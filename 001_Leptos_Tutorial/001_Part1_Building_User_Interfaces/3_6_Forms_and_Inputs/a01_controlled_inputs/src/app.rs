use leptos::{component, create_signal, event_target_value, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    } // start with a set of three rows
}
