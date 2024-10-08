use leptos::{component, create_signal, view, IntoView, SignalGet, SignalUpdate};

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
fn App() -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    // we'll talk more about them later
    let (count, set_count) = create_signal(0);

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! {
        <button
            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`

            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static
            on:click=move |_| {
                set_count.update(|n| *n += 2);
            }
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me"
        </button>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you write {count()}, this will *not* be reactive
            // it simply gets the value of count once
            {count()}
        </p>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount_to_body(App)
}
