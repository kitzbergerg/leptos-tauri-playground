use leptos::{
    component, create_action, create_node_ref, html::Input, ErrorBoundary, IntoView, Scope,
};
use leptos_macro::view;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use shared_model::FileWriterArgs;

use crate::{app::try_invoke, components::error_template::ErrorTemplate};

#[derive(Debug, Clone, thiserror::Error)]
enum Error {
    #[error("Call to tauri went wrong: {0}")]
    Boundary(String),
}

#[derive(Serialize, Deserialize)]
struct Args {
    content: FileWriterArgs,
}

#[component]
pub fn FileWriter(cx: Scope) -> impl IntoView {
    let file_writer_action = create_action(cx, move |(content, should_error): &(String, bool)| {
        let content = content.to_owned();
        let should_error = *should_error;
        async move { write_to_file(content, should_error).await }
    });

    let content_ref = create_node_ref::<Input>(cx);
    let should_error_ref = create_node_ref::<Input>(cx);

    view! { cx,
        <form
            class="row"
            on:submit=move |ev| {
                ev.prevent_default();
                let content = content_ref.get().expect("input to exist");
                let should_error = should_error_ref.get().expect("input to exist");
                file_writer_action.dispatch((content.value(), should_error.checked()));
            }
        >
            <input
                id="write_to_file-input"
                placeholder="Enter a file content..."
                type="text"
                node_ref=content_ref
            />
            <div>
                <input
                    name="ShouldError"
                    type="checkbox"
                    node_ref=should_error_ref
                />
                <label for="ShouldError">Should error</label>
            </div>
            <button type="submit">"Write to file"</button>
        </form>

         <ErrorBoundary
            fallback=|cx, errors| view! { cx, <ErrorTemplate errors=errors/> }
        >
            <p><b>{move || file_writer_action.value() }</b></p>
        </ErrorBoundary>
    }
}

async fn write_to_file(content: String, should_error: bool) -> Result<String, Error> {
    let args = to_value(&Args {
        content: FileWriterArgs {
            content,
            should_error,
        },
    })
    .unwrap();

    match try_invoke("write_to_file", args).await {
        Ok(val) => Ok(val.as_string().unwrap()),
        Err(val) => Err(Error::Boundary(val.as_string().unwrap())),
    }
}
