use app::App;

mod app;
mod btnd;
mod content;
mod layout;

fn main() {
    leptos::mount_to_body(App)
}
