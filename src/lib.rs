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
        <div class="p-8 space-y-4">
            <h1>kayh.dev</h1>

            <Links/>
        </div>
    }
}

#[component]
fn Links() -> impl IntoView {
    view! {
        <div>
            <h2>Links</h2>

            <ul>
                <li>
                    <a href="https://github.com/kayhhh" target="_blank">
                        "GitHub"
                    </a>
                </li>
                <li>
                    <a href="https://twitter.com/kayh_online" target="_blank">
                        "Twitter"
                    </a>
                </li>
                <li>
                    <a href="https://mirror.xyz/kayh-online.eth" target="_blank">
                        "Blog"
                    </a>
                </li>
                <li>
                    <a href="https://myanimelist.net/profile/kayh_online" target="_blank">
                        "MyAnimeList"
                    </a>
                </li>
            </ul>
        </div>
    }
}
