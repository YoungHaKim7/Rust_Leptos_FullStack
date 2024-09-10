use leptos::{component, view, Children, IntoView};

#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}

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

            <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
                // these get passed to `children`
                "Some text"
                <span>"A span"</span>
            </TakesChildren>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
