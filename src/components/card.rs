use leptos::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! { <div class="relative bg-slate-800 rounded-md px-5 py-4 shadow">{children()}</div> }
}
