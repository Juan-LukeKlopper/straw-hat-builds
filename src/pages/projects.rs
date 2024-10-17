use crate::components::project::*;
use leptos::*;

pub fn project() -> impl IntoView {
    view! {
        <div class="w-full bg-gradient-to-r from-purple-500 from-10% via-sky-500 via-30% to-emerald-500 to-90% p-8 text-center ">
            <div class="max-w-8xl mx-auto p-8 my-8 text-gray-100 border-2 border-black rounded ring ring-offset-2 ring-slate-900 bg-slate-900 text-lg text-justify">
                <ProjectOutput/>
            </div>
        </div>
    }
}
