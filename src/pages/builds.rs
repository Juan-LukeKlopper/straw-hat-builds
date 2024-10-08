use crate::components::builds_list::*;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn builds() -> impl IntoView {
    view! {
        <div class="w-full bg-gradient-to-r from-purple-500 from-10% via-sky-500 via-30% to-emerald-500 to-90% p-8 text-center ">
        <div class="max-w-8xl mx-auto p-8 my-8 text-gray-100 border-2 border-black rounded ring ring-offset-2 ring-slate-900 bg-slate-900 text-lg text-justify">
        <h1>"Welcome to our builds page!"</h1>
        <p>"Gaze apon all of the treasure maps we have collected!"</p>
        <p>"Click Learn More to start your expedition"</p>
        <BuildsList/>
        </div>
        </div>
    }
}
