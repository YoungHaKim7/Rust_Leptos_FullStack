use leptos::{component, create_signal, provide_context, view, IntoView};

use crate::layout::Layout;

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    provide_context(set_toggled);
    view! {
        <p>"Toggled? " {toggled}</p>
        <Layout set_toggled/>
    }
}
