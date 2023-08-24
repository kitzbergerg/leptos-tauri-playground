use leptos::{
    component, create_signal, ev::SubmitEvent, event_target_value, spawn_local, ErrorBoundary,
    IntoView, Scope, SignalGet, SignalSet, SignalGetUntracked,
};
use leptos_macro::view;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::{app::try_invoke, components::error_template::ErrorTemplate};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct FileContentArgs<'a> {
    content: &'a str,
}

#[derive(Debug, Clone, thiserror::Error)]
enum Error {
    #[error("Call to tauri went wrong: {0}")]
    Boundary(String),
}

#[component]
pub fn FileWriter(cx: Scope) -> impl IntoView {
    let (file_content, set_file_content) = create_signal(cx, String::new());
    let (write_to_file_msg, set_write_to_file_msg) = create_signal(cx, Ok(String::new()));

    let update_file_content = move |ev| {
        let v = event_target_value(&ev);
        set_file_content.set(v);
    };

    let write_to_file = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let args = to_value(&FileContentArgs {
                content: &file_content.get_untracked(),
            })
            .unwrap();
            let new_msg = match try_invoke("write_to_file", args).await {
                Ok(val) => Ok(val.as_string().unwrap()),
                Err(val) => Err(Error::Boundary(val.as_string().unwrap())),
            };
            set_write_to_file_msg.set(new_msg);
        });
    };

    view! { cx,
        <form class="row" on:submit=write_to_file>
            <input
                id="write_to_file-input"
                placeholder="Enter a file content..."
                on:input=update_file_content
            />
            <button type="submit">"Write to file"</button>
        </form>

         <ErrorBoundary
            fallback=|cx, errors| view! { cx, <ErrorTemplate errors=errors/> }
        >
            <p><b>{move || write_to_file_msg.get()}</b></p>
        </ErrorBoundary>
    }
}
