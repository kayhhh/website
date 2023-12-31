use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|| {
        view! { <App /> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                WriteSignal::set(&set_count, ReadSignal::get(&count) + 1);
            }
        >
            "Click me: "
            {move || ReadSignal::get(&count)}
        </button>
    }
}
