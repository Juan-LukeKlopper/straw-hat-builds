use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("About", "/about"), ("Builds", "/builds")];

    view! {
        <header class="relative z-50">
            <nav
                class="mx-auto flex w-full items-center justify-between p-6 lg:px-8 bg-gray-900"
                aria-label="Global"
            >
                <a href="/" class="-m-1.5 p-1.5">
                    <span class="sr-only">"StrawHatBuilds"</span>
                    <img
                        class="h-8 w-auto rounded-full"
                        src="Luffy_Default_Icon.jpeg"
                        alt="avatar"
                        width="64"
                        height="64"
                        href="/"
                    />
                </a>
                <div class="flex gap-x-6 lg:gap-x-12">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="text-sm font-semibold leading-6 text-gray-300"
                                    href=href.to_string()
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </nav>
        </header>
    }
}
