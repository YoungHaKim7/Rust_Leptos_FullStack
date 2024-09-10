use leptos::{component, view, Children, CollectView, IntoView};

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // Fragment has `nodes` field that contains a Vec<View>
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! {
        <ul>{children}</ul>
    }
}

#[component]
fn App() -> impl IntoView {
    // let (value, set_value) = create_signal(0);

    view! {
        <main>
            <h1>Manipulating CHildren</h1>

                <WrapsChildren>
                    "A"
                    "B"
                    "C"
                </WrapsChildren>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
