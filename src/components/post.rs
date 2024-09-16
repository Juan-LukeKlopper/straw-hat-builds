use leptos::*;

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use pulldown_cmark::{Options, Parser};
    let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    println!("title: {:?}", title);
    println!("html_output: {:?}", html_output);
    Ok(())
}

#[server(GetAvailablePosts, "/api")]
pub async fn get_available_posts() -> Result<Vec<String>, ServerFnError> {
    use std::fs;
    let content_dir = "./content"; // adjust this to your actual content directory path
    let mut available_posts = Vec::new();

    if let Ok(entries) = fs::read_dir(content_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                if entry.path().is_dir() {
                    available_posts.push(file_name);
                }
            }
        }
    }

    println!("Available_posts = {:?}", available_posts);
    Ok(available_posts)
}

#[component]
pub fn Posts() -> impl IntoView {
    let (posts, set_posts) = create_signal(vec!["".to_string()]);

    let async_projects = create_resource(
        // the first is the "source signal"
        posts,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { get_available_posts().await.unwrap() },
    );

    let async_result = move || {
        async_projects.get().map(|projects| {
            view! {
                <div>
                    {projects.iter().map(|project| {
                        view! {
                            <div class="max-w-xs p-6 my-4 text-gray-100 border-2 border-black rounded ring ring-offset-2 ring-slate-900 bg-gradient-to-b from-purple-500 from-10% via-sky-500 via-30% to-emerald-500 to-90% text-lg text-center">
                                <h2 class="text-2xl font-bold mb-4">{project}</h2>
                                <p class="text-base">"This is a brief description of the project."</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }
        })
    };

    let loading = async_projects.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                {move || async_result().unwrap_or_else(|| view! { <div><li>"Loading..."</li></div> })}
            </Suspense>
        </div>
    }
}

