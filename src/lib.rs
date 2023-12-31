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

                <span class="col-span-2">
                    <Card>
                        <HotlineWebring/>
                    </Card>
                </span>

                <Card>
                    <NavLinkAd/>
                </Card>

                <span class="row-span-2">
                    <Card>
                        <Links/>
                    </Card>
                </span>

                <Card>
                    <Projects/>
                </Card>

                <span class="col-span-2">
                    <Card>
                        <RetroButtons/>
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
                    <a href="https://x.com/kayh_online" target="_blank">
                        "[X]"
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
            <a href="https://hotlinewebring.club/kayh/previous">"[prev]"</a>
            <p class="text-amber-300">"Hotline Webring"</p>
            <a href="https://hotlinewebring.club/kayh/next">"[next]"</a>
        </div>
    }
}

#[component]
fn NavLinkAd() -> impl IntoView {
    view! {
        <iframe
            src="https://dimden.neocities.org/navlink/"
            title="NavLink Ad System"
            width="180"
            height="180"
            class="border-none mx-auto"
        ></iframe>
    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <div>
            <h2>"Projects"</h2>

            <ul>
                <li>
                    <a href="https://github.com/unavi-xyz/unavi" target="_blank">
                        "[UNAVI]"
                    </a>
                </li>
                <li>
                    <a href="https://github.com/unavi-xyz/wired-protocol" target="_blank">
                        "[The Wired]"
                    </a>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn RetroButtons() -> impl IntoView {
    view! {
        <div class="flex flex-wrap">
            <img src="/public/retro/miku.gif" alt="this site is MIKU APPROVED" width=88 height=31/>
            <img src="/public/retro/lain.gif" alt="Lain" width=88 height=31/>
            <img src="/public/retro/ilikecomputer.png" alt="I LIKE COMPUTER" width=88 height=31/>
            <img src="/public/retro/jquery.gif" alt="jQuery is evil!" width=88 height=31/>
            <img src="/public/retro/buyit.gif" alt="MUG ROOT BEET" width=88 height=31/>

            <a href="https://archive.org/" target="_blank">
                <img
                    src="/public/retro/internetarchive.gif"
                    alt="Internet Archive"
                    width=88
                    height=31
                />
            </a>
            <a href="https://www.tanguy.cyou/" target="_blank">
                <img
                    src="https://www.tanguy.cyou/assets/img/links/button.webp"
                    alt="tanguy.cyou"
                    width=88
                    height=31
                />
            </a>
            <a href="https://www.melody.my.id/" target="_blank">
                <img
                    src="https://melodies.neocities.org/image/button/melody.png"
                    alt="Melody Hot Springs"
                    width=88
                    height=31
                />
            </a>
            <a href="https://dimden.dev/" target="_blank">
                <img
                    src="https://dimden.dev/services/images/88x31.gif"
                    alt="DIMDEN"
                    width=88
                    height=31
                />
            </a>
            <a href="https://melankorin.net/" target="_blank">
                <img
                    src="https://melankorin.net/assets/img/buttons/button-1.gif"
                    alt="melankorin"
                    width=88
                    height=31
                />
            </a>
        </div>
    }
}
