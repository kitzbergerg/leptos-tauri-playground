use std::collections::HashMap;

use leptos::{create_rw_signal, provide_context, IntoView, RwSignal};
use leptos_macro::{component, view};
use uuid::Uuid;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::components::{
    error::{Error, ErrorToast},
    file_writer::FileWriter,
    greet::Greet,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke", catch)]
    pub async fn try_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub errors: RwSignal<HashMap<Uuid, Error>>,
}
impl GlobalState {
    pub fn new() -> Self {
        Self {
            errors: create_rw_signal(HashMap::new()),
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(GlobalState::new());

    view! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <Greet/>

            <br/>

            <FileWriter/>

            <ErrorToast/>
        </main>
    }
}
