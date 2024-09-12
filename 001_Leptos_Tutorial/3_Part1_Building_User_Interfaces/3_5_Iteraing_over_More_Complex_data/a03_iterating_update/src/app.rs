use leptos::{component, create_rw_signal, create_signal, logging, view, For, IntoView, SignalGet};

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            // log the new value of the signal
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}
