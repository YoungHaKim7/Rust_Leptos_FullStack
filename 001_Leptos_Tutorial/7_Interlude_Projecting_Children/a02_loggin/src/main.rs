use std::rc::Rc;

use leptos::{component, store_value, view, ChildrenFn, Fragment, IntoView, Show, Suspense};

#[component]
pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let fallback = store_value(fallback);
    let children = store_value(children);
    view! {
        <Suspense
            fallback=|| ()
        >
            <Show
                when=|| todo!()
                fallback=move || fallback.with_value(|fallback| fallback())
            >
                {children.with_value(|children| children())}
            </Show>
        </Suspense>
    }
}

#[component]
fn App() -> impl IntoView {
    let name = "Alice".to_string();

    view! {
        <LoggedIn
            fallback=|| {
                view! {
                    <p>Login to view content</p>
                }
            }
        children=Rc::new(move || {
            Fragment::new(vec![
                // view! {
                //     <p>
                //         <Outer>
                //             <Inner>
                //                 <Inmost name=name.clone()/>
                //             </Inner>
                //         </Outer>
                //     </p>
                // }.into_view()
            ])
        })
    />
    }
}

#[component]
fn Outer(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
fn Inner(children: ChildrenFn) -> impl IntoView {
    children()
}

#[component]
fn Inmost(name: String) -> impl IntoView {
    view! {
        <p>{name}</p>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
