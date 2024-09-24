use leptos::*;

use crate::app::{
    components::{DashboardHeader, Header},
    server_functions::get_persons,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let get_persons_info = create_resource(|| (), |_| async move { get_persons().await });

    view! {
        <body class="bg-gray-900 overflow-x-hide">
            <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
                <Header />
                <DashboardHeader />
            </div>
        </body>
    }
}
