use leptos::{
    component, create_signal, ev::SubmitEvent, event_target_value, spawn_local, ErrorBoundary,
    IntoView, Scope, SignalGet, SignalGetUntracked, SignalSet, event_target_checked,
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
struct FileWriterArgs {
    content: Args,
}

#[derive(Serialize, Deserialize)]
struct Args {
    content: String,
    should_error: bool,
}

#[derive(Debug, Clone, thiserror::Error)]
enum Error {
    #[error("Call to tauri went wrong: {0}")]
    Boundary(String),
}

#[component]
pub fn FileWriter(cx: Scope) -> impl IntoView {
    let (file_content, set_file_content) = create_signal(cx, String::new());
    let (should_error, set_should_error) = create_signal(cx, false);
    let (write_to_file_msg, set_write_to_file_msg) = create_signal(cx, Ok(String::new()));

    let update_file_content = move |ev| {
        let v = event_target_value(&ev);
        set_file_content.set(v);
    };
    let update_should_error = move |ev| {
        let v = event_target_checked(&ev);
        set_should_error.set(v);
    };

    let write_to_file = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let args = to_value(&FileWriterArgs {
                content: Args {
                    content: file_content.get_untracked(),
                    should_error: should_error.get_untracked(),
                },
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
            <div>
                <input 
                    type="checkbox"
                    name="ShouldError"
                    on:input=update_should_error
                />
                <label for="ShouldError">Should error</label>
            </div>
            <button type="submit">"Write to file"</button>
        </form>

         <ErrorBoundary
            fallback=|cx, errors| view! { cx, <ErrorTemplate errors=errors/> }
        >
            <p><b>{move || write_to_file_msg.get()}</b></p>
        </ErrorBoundary>
    }
}
