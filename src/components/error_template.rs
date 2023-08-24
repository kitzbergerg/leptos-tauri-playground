use leptos::{CollectView, Errors, IntoView, RwSignal, Scope, SignalGet};
use leptos_macro::{component, view};

#[component]
pub fn ErrorTemplate(cx: Scope, errors: RwSignal<Errors>) -> impl IntoView {
    view! { cx,
        <div class="error">
            {move || errors.get()
                .into_iter()
                .map(|(_, e)| view! { cx, <p>{e.to_string()}</p>})
                .collect_view(cx)
            }
        </div>
    }
}
