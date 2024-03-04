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
        <div class="max-w-2xl mx-auto py-8 px-4">
            <h1 class="text-center pb-2">"kayh.dev"</h1>

            <Nav/>

            <div class="grid grid-cols-1 md:grid-cols-auto gap-4">
                <span class="md:col-span-2">
                    <Card>
                        <About/>

                        <img
                            src="/public/neko-arc-3d.png"
                            alt="Neko Arc 3D"
                            width=120
                            height=213
                            class="absolute -right-[160px] -top-[80px] animate-float"
                        />
                    </Card>
                </span>

                <span class="md:col-span-2">
                    <Card>
                        <HotlineWebring/>
                    </Card>
                </span>

                <Card>
                    <img
                        src="https://count.getloli.com/get/@kayh"
                        alt="Site View Count"
                        width=283
                        height=90
                        class="mx-auto"
                    />

                    <img
                        src="/public/neko-arc-lookback.gif"
                        alt="Neko Arc"
                        width=160
                        height=160
                        class="absolute -left-[160px] -top-[50px] animate-float2"
                    />
                </Card>

                <Card>
                    <Socials/>
                </Card>

                <Card>
                    <Projects/>
                </Card>

                <Card>
                    <NavLinkAd/>
                </Card>

                <Card>
                    <Music/>
                </Card>

                <Card>
                    <SiteButton/>
                </Card>

                <span class="md:col-span-2">
                    <Card>
                        <RetroButtons/>
                    </Card>
                </span>
            </div>
        </div>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav class="flex justify-center space-x-2 pb-4">
            <a href="/">"[Home]"</a>
            <a href="https://mirror.xyz/kayh-online.eth">"[Blog]"</a>
        </nav>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div>
            <h2 class="text-blue-400">"About"</h2>

            <p>
                "Hi, I'm Kayh! Welcome to my site! I'm a self-taught developer building things I think are cool. I play a lot of VRChat, so if you see me around come say hi!"
            </p>
        </div>
    }
}

#[component]
fn Socials() -> impl IntoView {
    view! {
        <div>
            <h2 class="text-green-400">"Socials"</h2>

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
                    <a href="https://myanimelist.net/profile/kayh_online" target="_blank">
                        "[MAL]"
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
            <h2 class="text-purple-400">"Projects"</h2>

            <ul>
                <li>
                    <a href="https://unavi.xyz" target="_blank">
                        "[UNAVI]"
                    </a>
                </li>
                <li>
                    <a href="https://wired-protocol.org" target="_blank">
                        "[The Wired]"
                    </a>
                </li>
                <li>
                    <a href="https://vrm-viewer.kayh.dev/">"[VRM Viewer]"</a>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn RetroButtons() -> impl IntoView {
    view! {
        <div class="flex flex-wrap justify-center">
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

#[component]
fn Music() -> impl IntoView {
    view! {
        <div class="h-full flex justify-center items-center">
            <audio src="/public/lucki-2021-vibes.mp3" controls autoplay></audio>

            <script>
                let audio = document.querySelector("audio");
                audio.volume = 0.4;
            </script>
        </div>
    }
}

#[component]
fn SiteButton() -> impl IntoView {
    view! {
        <div class="p-2 space-y-2 flex flex-col items-center rounded">
            <img src="/public/btn.gif" alt="kayh.dev" width=88 height=31/>

            <textarea
                id="btn-code"
                disabled
                title="Copy this to your site!"
                class="w-full rounded bg-black text-xs p-1"
            >
                r#"<a href="https://kayh.dev" target="_blank"><img src="https://kayh.dev/public/btn.gif" alt="kayh.dev" width="88px" height="31px" /></a>"#
            </textarea>
        </div>
    }
}
