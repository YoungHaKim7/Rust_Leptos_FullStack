use leptos::{component, view, IntoView};

use crate::{
    controlledcomponent::ControlledComponent, uncontrolledcomponent::UncontrolledComponent,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
        <h2>"Uncontrolled Component"</h2>
        <UncontrolledComponent/>
    }
}
