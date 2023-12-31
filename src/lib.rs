use leptos::*;
use wasm_bindgen::prelude::*;

mod components;

use components::*;

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="max-w-2xl mx-auto py-8 grid gap-4">
            <h1 class="text-center">"kayh.dev"</h1>

            <div class="grid gap-4">
                <span class="col-span-2">
                    <Card>
                        <About/>
                    </Card>
                </span>

                <Card>
                    <HotlineWebring/>
                </Card>

                <span class="row-span-2">
                    <Card>
                        <Links/>
                    </Card>
                </span>
            </div>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h2>"About"</h2>

            <p>
                "Hi, I'm Kayh! Welcome to my site! I'm a self-taught developer building things I think are cool. I play a lot of VRChat, so if you see me around come say hi!"
            </p>
        </div>
    }
}

#[component]
fn Links() -> impl IntoView {
    view! {
        <div>
            <h2>"Links"</h2>

            <ul>
                <li>
                    <a href="https://github.com/kayhhh" target="_blank">
                        "[GitHub]"
                    </a>
                </li>
                <li>
                    <a href="https://twitter.com/kayh_online" target="_blank">
                        "[Twitter]"
                    </a>
                </li>
                <li>
                    <a href="https://mirror.xyz/kayh-online.eth" target="_blank">
                        "[Blog]"
                    </a>
                </li>
                <li>
                    <a href="https://myanimelist.net/profile/kayh_online" target="_blank">
                        "[MyAnimeList]"
                    </a>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn HotlineWebring() -> impl IntoView {
    view! {
        <div class="flex justify-between space-x-4">
            <a href="https://hotlinewebring.club/kayh/previous">"<---"</a>
            <p class="text-amber-300">Hotline Webring</p>
            <a href="https://hotlinewebring.club/kayh/next">"--->"</a>
        </div>
    }
}
