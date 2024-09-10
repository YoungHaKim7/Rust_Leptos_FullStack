use leptos::{component, view, IntoView};

#[component]
fn App() -> impl IntoView {
    // let (value, set_value) = create_signal(0);

    view! {
        <main>
            <h1>Component Children</h1>

            <form>
                <fieldset>
                    <label>
                        "Some Input"
                        <input type="text" name="something"/>
                    </label>
                </fieldset>
                <button>"Submit"</button>
            </form>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
