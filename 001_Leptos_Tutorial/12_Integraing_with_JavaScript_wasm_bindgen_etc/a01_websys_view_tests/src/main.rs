// use leptos::{component, mount_to_body, view};

// #[component]

// fn SelfUpdatingEffect() -> Element {
//     let (a, set_a) = create_signal(false);

//     create_effect(move |_| {
//         if !a() {
//             set_a(true);
//         }
//     });

//     view! {
//       <h1>"Hello " {move || a().to_string()}</h1>
//     }
// }

// #[component]
// fn Tests() -> Element {
//     view! {

//         <div>
//             <div><SelfUpdatingEffect/></div>
//             <div><BlockOrders/></div>
//             //<div><TemplateConsumer/></div>
//         </div>
//     }
// }

// #[component]
// fn BlockOrders() -> Element {
//     let a = "A";
//     let b = "B";
//     let c = "C";

//     view! {

//         <div>
//             <div>"A"</div>
//             <div>{a}</div>
//             <div><span>"A"</span></div>
//             <div><span>{a}</span></div>
//             <hr/>
//             <div>"A" {b}</div>
//             <div>{a} "B"</div>
//             <div>{a} {b}</div>
//             <div>{"A"} {"B"}</div>
//             <div><span style="color: red">{a}</span> {b}</div>
//             <hr/>
//             <div>{a} "B" {c}</div>
//             <div>"A" {b} "C"</div>
//             <div>{a} {b} "C"</div>
//             <div>{a} {b} {c}</div>
//             <div>"A" {b} {c}</div>
//             <hr/>
//             <div>"A" {b} <span style="color: red">"C"</span></div>
//             <div>"A" {b} <span style="color: red">{c}</span></div>
//             <div>"A" <span style="color: red">"B"</span> "C"</div>
//             <div>"A" <span style="color: red">"B"</span> {c}</div>
//             <div>{a} <span style="color: red">{b}</span> {c}</div>
//             <div>"A" {b} <span style="color: red">{c}</span></div>
//             <div><span style="color: red">"A"</span> {b} {c}</div>
//             <div><span style="color: red">{a}</span> "B" {c}</div>
//             <div><span style="color: red">"A"</span> {b} "C"</div>
//             <hr/>
//             <div><span style="color: red">"A"</span> <span style="color: blue">{b}</span> {c}</div>
//             <div><span style="color: red">{a}</span> "B" <span style="color: blue">{c}</span></div>
//             <div><span style="color: red">"A"</span> {b} <span style="color: blue">"C"</span></div>
//             <hr/>
//             <div><A/></div>
//             <div>"A" <B/></div>
//             <div>{a} <B/></div>
//             <div><A/> "B"</div>
//             <div><A/> {b}</div>
//             <div><A/><B/></div>
//             <hr/>
//             <div><A/> "B" <C/></div>
//             <div><A/> {b} <C/></div>
//             <div><A/> {b} "C"</div>
//         </div>
//     }
// }

// #[component]
// fn A() -> Element {
//     view! { <span style="color: red">"A"</span> }
// }

// #[component]
// fn B() -> Element {
//     view! { <span style="color: red">"B"</span> }
// }

// #[component]
// fn C() -> Element {
//     view! { <span style="color: red">"C"</span> }
// }

// #[component]
// fn TemplateConsumer() -> Element {
//     let tpl = view! { <TemplateExample/> };
//     let cloned_tpl = tpl
//         .unchecked_ref::<web_sys::HtmlTemplateElement>()
//         .content()
//         .clone_node_with_deep(true)
//         .expect("couldn't clone template node");

//     view! {

//         <div id="template">
//             /* <h1>"Template Consumer"</h1>
//             {cloned_tpl} */
//         </div>
//     }
// }

// #[component]
// fn TemplateExample() -> Element {
//     view! {

//         <template>
//             <div>"Template contents"</div>
//         </template>
//     }
// }

// pub fn main() {
//     _ = console_log::init_with_level(log::Level::Debug);
//     console_error_panic_hook::set_once();
//     mount_to_body(|| view! { <Tests/> })
// }

use std::ops::Deref;

use leptos::component;
use leptos::html;
use leptos::html::ElementDescriptor;
use leptos::view;
use leptos::Children;
use leptos::IntoView;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CssStyleSheet;
use web_sys::HtmlElement;
use web_sys::ShadowRootInit;
use web_sys::ShadowRootMode;

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let host = html::main;
    let styles = vec![
        r###"
        :host {
            font-family: "Roboto";
            color: orange;
        }
        "###,
    ];
    view! {
        <Shadow host styles>
            <UnprotectedParagraph/>
            <ProtectedParagraph/>
        </Shadow>
    }
}

#[component]
fn UnprotectedParagraph() -> impl IntoView {
    view! { <p>"Styled text!!"</p> }
}

#[component]
fn ProtectedParagraph() -> impl IntoView {
    let host = html::div;
    // Prevent the color and font styles from piercing shadow DOM.
    let styles = vec![
        r###"div {
            all: initial;
        }"###,
    ];
    view! {
        <Shadow host styles>
            <div>
                <p>"Plaintext..."</p>
            </div>
        </Shadow>
    }
}

#[component]
fn Shadow<F, Elem, IntoElem>(
    host: F,
    #[prop(default = vec![])] styles: Vec<&'static str>,
    children: Children,
) -> impl IntoView
where
    F: FnOnce() -> html::HtmlElement<Elem>,
    Elem: ElementDescriptor + Deref<Target = IntoElem>,
    IntoElem: JsCast,
{
    let host = host();
    let shadow_host: &web_sys::Element = host.deref().unchecked_ref();
    let shadow_root = shadow_host
        .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
        .unwrap();
    {
        let shadow_root: &ShadowRootExt = shadow_root.unchecked_ref();
        let sheets = shadow_root.adopted_style_sheets();
        for &style_str in styles.iter() {
            let style = CssStyleSheet::new().expect("construct stylesheet");
            style
                .replace_sync(style_str)
                .expect("add style rule to constructed stylesheet");
            sheets.push(&style);
        }
    }
    leptos::mount_to(shadow_root.unchecked_into::<HtmlElement>(), children);
    host
}

#[wasm_bindgen]
extern "C" {
    pub type ShadowRootExt;
    pub type StyleSheetList;

    #[wasm_bindgen(method, getter = adoptedStyleSheets)]
    pub fn adopted_style_sheets(this: &ShadowRootExt) -> StyleSheetList;

    #[wasm_bindgen(method)]
    pub fn push(this: &StyleSheetList, value: &JsValue);
}
