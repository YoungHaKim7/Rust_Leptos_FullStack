use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};

use crate::formexample::FormExample;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <h1><code>"<Form/>"</code></h1>
            <main>
                <Routes>
                    <Route path="" view=FormExample/>
                </Routes>
            </main>
        </Router>
    }
}
