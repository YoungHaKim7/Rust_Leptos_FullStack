use leptos::{component, view, IntoView};

#[component]
fn App() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
    // this will just render "012"
    <p>{values.clone()}</p>
    // or we can wrap them in <li>
    <ul>
        {values.into_iter()
            .map(|n| view! { <li>{n}test</li>})
            .collect::<Vec<_>>()}
    </ul>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
