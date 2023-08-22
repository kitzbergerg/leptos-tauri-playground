use leptos::{
    component, create_signal, ev::SubmitEvent, event_target_value, spawn_local, IntoView, Scope,
    SignalGet, SignalSet,
};
use leptos_macro::view;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::app::invoke;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct FileContentArgs<'a> {
    content: &'a str,
}
#[component]
pub fn FileWriter(cx: Scope) -> impl IntoView {
    let (file_content, set_file_content) = create_signal(cx, String::new());
    let (write_to_file_msg, set_write_to_file_msg) = create_signal(cx, String::new());

    let update_file_content = move |ev| {
        let v = event_target_value(&ev);
        set_file_content.set(v);
    };

    let write_to_file = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let args = to_value(&FileContentArgs {
                content: &file_content.get(),
            })
            .unwrap();
            let new_msg = invoke("write_to_file", args).await.as_string().unwrap();
            set_write_to_file_msg.set(new_msg);
        });
    };

    view! { cx,
        <div>
            <form class="row" on:submit=write_to_file>
            <input
                id="write_to_file-input"
                placeholder="Enter a file content..."
                on:input=update_file_content
            />
                <button type="submit">"Write to file"</button>
            </form>

            <p><b>{ move || write_to_file_msg.get() }</b></p>
        </div>
    }
}
