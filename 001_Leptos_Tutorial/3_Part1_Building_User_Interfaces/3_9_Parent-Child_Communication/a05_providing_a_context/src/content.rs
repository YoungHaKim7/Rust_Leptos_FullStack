use leptos::{component, view, IntoView, WriteSignal};

use crate::btnd::ButtonD;

#[component]
pub fn Content(set_toggled: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div class = "content">
            <ButtonD />
        </div>
    }
}
