use leptos::{component, create_action, create_node_ref, html::Input, IntoView};
use leptos_macro::view;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use crate::app::invoke;

#[derive(Serialize, Deserialize)]
struct Args<'a> {
    name: &'a str,
}

#[component]
pub fn Greet() -> impl IntoView {
    let greet_action = create_action(|input: &String| {
        let input = input.to_owned();
        async move { greet(&input).await }
    });

    let input_ref = create_node_ref::<Input>();

    view! {
        <form
            class="row"
            on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                greet_action.dispatch(input.value());
            }
        >
            <input
                id="greet-input"
                placeholder="Enter a name..."
                type="text"
                node_ref=input_ref
            />
            <button type="submit">"Greet"</button>
        </form>
        <p><b>{ move || greet_action.value() }</b></p>
    }
}

async fn greet(name: &str) -> String {
    let args = to_value(&Args { name }).unwrap();
    invoke("greet", args).await.as_string().unwrap()
}
