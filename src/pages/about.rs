use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="w-full bg-gradient-to-r from-purple-500 from-10% via-sky-500 via-30% to-emerald-500 to-90% p-8 ">

        <div class="max-w-8xl mx-auto p-8 my-8 text-gray-100 border-2 border-black rounded ring ring-offset-2 ring-slate-900 bg-slate-900 text-lg text-justify">
            <h1 class="text-4xl font-bold mb-6 text-center">"About Straw-Hat Builds"</h1>
            <p class="my-8 ">
                <strong>"Wealth, Fame, Power."</strong>" These words ignited an era of exploration, much like how "<strong>"Straw-hat Build"</strong>" seeks to inspire a journey of creation and growth. This platform isn’t just about code; it’s about the pursuit of mastery through building, shipping, and releasing real products. Inspired by "<a href="https://buildspace.so"><strong>"Buildspace"</strong></a>", we honor their spirit by helping developers transcend tutorials and push their work into the world."
            </p>
            <p class="my-8 ">
                "At "<strong>"Straw-Hat Builds"</strong>" we’re driven by the idea that meaningful work happens when you step beyond the confines of localhost. Creating and releasing tangible products brings a unique sense of accomplishment—a testament to your dedication. This platform serves as a tool to help you turn your knowledge into real-world value, something you can add to your resume, something others can click, explore, and interact with."
            </p>
            <p class="my-8 ">
                "In life, as in "<strong>"One Piece"</strong>", there’s a deeper purpose in the journey. Just as "<strong>"Gol D. Roger"</strong>" launched the Great Pirate Era, we aim to spark your journey towards real-world applications, where your creations aren’t just artifacts of learning but living, breathing entities. These are not just lines of code but "<strong>"experiences"</strong>" apps, websites, tools that serve real users."
            </p>


            <p class="my-8 ">
                "This is about "<strong>"taking responsibility"</strong>" for your own progress. The difference between potential and reality is found in the act of shipping your work, making it accessible to others. This process—of building, of releasing—isn’t easy. It’s not meant to be. But it’s how you grow, both as a developer and as a person."
            </p>

            <p class="my-8 ">
                "So if you’re a builder, dreamer, or someone eager to learn, this is where your journey begins. Welcome aboard!"
            </p>
        </div>
    </div>
        }
}
