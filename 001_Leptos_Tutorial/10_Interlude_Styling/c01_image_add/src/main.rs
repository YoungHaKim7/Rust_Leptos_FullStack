use leptos::{component, view, IntoView};
// use leptos_image::Image;
use stylance::import_style;

import_style!(style, "main.module.scss");

// #[component]
// pub fn MyImage() -> impl IntoView {
//     view! {
//         <Image
//             src="/imgs/test.jpg"
//             blur=true
//             width=750
//             height=500
//             quality=85
//         />
//     }
// }

#[component]
fn App() -> impl IntoView {
    // leptos_image::provide_image_context();
    view! {
       <p class=style::one>"test p scss"</p>
           <div class="one-"> one BackGround color Blue</div>
               <h1 id="two">"Hello"</h1>
               <h2 class="three-">"World"</h2>
               <h3 class="red-">"friends!" </h3>
            <figure>
                <img src="test.jpg" class="test-image-"/>
                <p>test test teset good imgs</p>
            </figure>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
