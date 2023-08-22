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

#[component]
pub fn Greet(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
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

    view! { cx,
        <form class="row" on:submit=greet>
            <input
                id="greet-input"
                placeholder="Enter a name..."
                on:input=update_name
            />
            <button type="submit">"Greet"</button>
        </form>

        <p><b>{ move || greet_msg.get() }</b></p>
    }
}
