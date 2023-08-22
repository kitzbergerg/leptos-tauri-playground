use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct FileContentArgs<'a> {
    content: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let (file_content, set_file_content) = create_signal(cx, String::new());
    let (write_to_file_msg, set_write_to_file_msg) = create_signal(cx, String::new());


    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let update_file_content = move |ev| {
        let v = event_target_value(&ev);
        set_file_content.set(v);
    };


    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let write_to_file = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if file_content.get().is_empty() {
                return;
            }

            let args = to_value(&FileContentArgs { content: &file_content.get() }).unwrap();
            let new_msg = invoke("write_to_file", args).await.as_string().unwrap();
            set_write_to_file_msg.set(new_msg);
        });
    };

    view! { cx,
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

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>

            <br/>

            <form class="row" on:submit=write_to_file>
                <input
                    id="write_to_file-input"
                    placeholder="Enter a file content..."
                    on:input=update_file_content
                />
                <button type="submit">"Write to file"</button>
            </form>

            <p><b>{ move || write_to_file_msg.get() }</b></p>
        </main>
    }
}
