use leptos::{expect_context, For, IntoView, Scope, SignalGet, SignalUpdate};
use leptos_macro::{component, view};
use uuid::Uuid;

use crate::app::GlobalState;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Call to tauri went wrong: {0}")]
    Boundary(String),
}

impl Error {
    pub fn show(self, cx: Scope) {
        let errors = expect_context::<GlobalState>(cx).errors;
        errors.update(|map| {
            map.insert(Uuid::new_v4(), self);
        });
    }
}

#[component]
pub fn ErrorToast(cx: Scope) -> impl IntoView {
    let errors = expect_context::<GlobalState>(cx).errors;

    view! { cx,
        <div class="error-container">
            <For
                each=move || errors.get()
                key=|error| error.0
                view=move |cx, (id, error)| {
                    view! { cx,
                        <div
                            class="error"
                            key={id.to_string()}
                            on:click=move |_| errors.update(|map| { map.remove(&id); })
                        >
                            <p>{error.to_string()}</p>
                        </div>
                    }
                }
            />
        </div>
    }
}
