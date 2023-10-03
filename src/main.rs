use leptos::mount_to_body;
use leptos_macro::view;

use crate::app::App;

mod app;
pub mod components;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
