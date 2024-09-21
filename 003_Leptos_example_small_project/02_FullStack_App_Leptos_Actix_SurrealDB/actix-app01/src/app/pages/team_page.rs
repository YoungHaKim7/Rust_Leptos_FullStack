use leptos::*;

use crate::app::components::Header;

#[component]
pub fn TeamPage() -> impl IntoView {
    view! {
        <body class="bg-gray-900 overflow-x-hide">
            <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
                <Header />
                "Team Page here"
            </div>
        </body>
    }
}
