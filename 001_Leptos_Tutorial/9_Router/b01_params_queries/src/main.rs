use crate::app::App;

mod app;
mod contactinfo;
mod contactlist;

fn main() {
    leptos::mount_to_body(App)
}
