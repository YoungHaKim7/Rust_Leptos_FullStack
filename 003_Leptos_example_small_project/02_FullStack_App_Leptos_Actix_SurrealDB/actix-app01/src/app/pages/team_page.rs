use leptos::*;

use crate::app::components::Header;

#[component]
pub fn TeamPage() -> impl IntoView {
    const ADD_BUTTON_STYLE: &str = "bg-[#7734e7] px-8 py-2 rounded text-white transition-all duration-1000 ease-in-out hover:bg-[#8448e9]";

    let (if_show_modal, set_if_show_modal) = create_signal(false);

    let on_click = move |_| {
        set_if_show_modal(!if_show_modal());
    };

    view! {
        <body class="bg-gray-900 overflow-x-hide">
            <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
                <Header />

                <div class="mt-20">

                    <div class="text-white flex flex-col w-full mx-auto items-center justify-center z-25">
                        <div class="flex flex-row flex-col w-full mx-auto items-center justify-center">

                            <div class="flex flex-row w-full max-w-[52rem]">
                                <div class="pr-4 mt-4 text-xl">
                                    "Members"
                                </div>
                                <hr class="w-full max-w-[48rem] pl-4 pr-4 pt-4 mt-8 mr-4" />
                                <button on:click=on_click class=ADD_BUTTON_STYLE>
                                    "Add"
                                </button>

                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </body>
    }
}
