use leptos::{component, store_value, view, ChildrenFn, IntoView, Show, Suspense};

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

// #[component]
// pub fn Outer(children: ChildrenFn) -> impl IntoView {
//     children()
// }

// #[component]
// pub fn Inner(children: ChildrenFn) -> impl IntoView {
//     children()
// }

// #[component]
// pub fn Inmost(name: String) -> impl IntoView {
//     view! {
//         <p>{name}</p>
//     }
// }

#[component]
fn App() -> impl IntoView {
    // let name = "Alice".to_string();

    view! {
            // <p>
            // <Outer>
            //     <Inner>
            //         <Inmost name=name.clone()/>
            //     </Inner>
            // </Outer>
            // </p>

    <div>
        Suspense(
            ::leptos::component_props_builder(&Suspense)
                .fallback(|| ())
                .children({
                    // fallback and children are moved into this closure
                    Box::new(move || {
                        {
                            // fallback and children captured here
                            leptos::Fragment::lazy(|| {
                                vec![
                                    (Show(
                                        ::leptos::component_props_builder(&Show)
                                            .when(|| true)
                                            // but fallback is moved into Show here
                                            .fallback(fallback)
                                            // and children is moved into Show here
                                            .children(children)
                                            .build(),
                                    )
                                    .into_view()),
                                ]
                            })
                        }
                    })
                })
                .build(),
        )
        </div>
        }
}

fn main() {
    leptos::mount_to_body(App)
}
