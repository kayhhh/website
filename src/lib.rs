use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-3xl text-red-500 bg-neutral-300 p-8">Leptos</h1>
        </div>
    }
}
