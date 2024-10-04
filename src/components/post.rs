use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
}

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
pub async fn get_available_posts() -> Result<Vec<Project>, ServerFnError> {
    use std::fs;
    let content_dir = "./content"; // Adjust this to your actual content directory path
    let mut available_posts = Vec::new();

    if let Ok(entries) = fs::read_dir(content_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let project_dir = entry.path();

                if project_dir.is_dir() {
                    // Default description if README.md doesn't exist
                    let mut description = "No description available.".to_string();

                    // Check if README.md exists in the project folder
                    let readme_path = project_dir.join("README.md");
                    if readme_path.exists() {
                        // Read the contents of README.md
                        if let Ok(content) = fs::read_to_string(readme_path) {
                            description = content;
                        }
                    }

                    // Push the project with name and description
                    available_posts.push(Project {
                        name: file_name,
                        description,
                    });
                }
            }
        }
    }

    println!("Available_projects = {:?}", available_posts);
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
                <div class="container mx-auto px-5 py-2 lg:px-32 lg:pt-12 space-y-6">
                    {projects.iter().map(|project| {
                        let background_image_url = format!("/backgrounds/{}.svg", project.name.replace(" ", "%20"));

                        view! {
                            <div class="flex h-[400px] overflow-hidden rounded-lg bg-gray-100">
                                <div
                                    class="w-1/2 h-full bg-center bg-no-repeat"
                                    style=format!("background-image: url('{}'); background-size: contain;", background_image_url)
                                ></div>

                                <div class="w-1/2 p-12 flex flex-col justify-center text-center bg-black text-white">
                                    <h2 class="mb-4 text-4xl font-semibold">{&project.name}</h2>
                                    <h4 class="mb-6 text-xl font-semibold">{&project.description}</h4>
                                    <button
                                      type="button"
                                      class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
                                    >
                                      Learn More
                                    </button>
                                </div>
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
