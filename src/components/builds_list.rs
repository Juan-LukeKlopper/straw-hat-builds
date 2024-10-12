use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub name: String,
    pub description: String,
}

#[server(Test, "/api")]
pub async fn test() -> Result<(), ServerFnError> {
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    // Load environment variables from the .env file
    dotenv().ok();

    // Get the DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Print the database URL to verify
    println!("Connecting to database with URL: {}", database_url);

    // Create a PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Connection is successful, proceed with your queries
    println!("Successfully connected to the database");

    // Explicitly close the pool when done
    pool.close().await;

    println!("Disconnected from the database");

    Ok(())
}

#[server(GetAvailableBuilds, "/api")]
pub async fn get_available_builds() -> Result<Vec<Build>, ServerFnError> {
    use std::fs;
    let content_dir = "./content"; // Adjust this to your actual content directory path
    let mut available_builds = Vec::new();

    if let Ok(entries) = fs::read_dir(content_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                let build_dir = entry.path();

                if build_dir.is_dir() {
                    // Default description if README.md doesn't exist
                    let mut description = "No description available.".to_string();

                    // Check if README.md exists in the project folder
                    let readme_path = build_dir.join("README.md");
                    if readme_path.exists() {
                        // Read the contents of README.md
                        if let Ok(content) = fs::read_to_string(readme_path) {
                            description = content;
                        }
                    }

                    // Push the project with name and description
                    available_builds.push(Build {
                        name: file_name,
                        description,
                    });
                }
            }
        }
    }

    Ok(available_builds)
}

#[component]
pub fn builds_list() -> impl IntoView {
    let (builds, _set_builds) = create_signal(vec!["".to_string()]);

    let async_builds = create_resource(
        // the first is the "source signal"
        builds,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_value| async move {
            let _ = test().await;
            get_available_builds().await.unwrap()
        },
    );

    let list_of_builds = move || {
        async_builds.get().map(|builds| {
            view! {
                <div class="container mx-auto px-5 py-2 lg:px-32 lg:pt-12 space-y-6">
                    {builds.iter().map(|build| {
                        let background_image_url = format!("/backgrounds/{}.svg", build.name.replace(" ", "%20"));

                        view! {
                            <div class="flex h-[400px] overflow-hidden rounded-lg bg-gray-100">
                                <div
                                    class="w-1/2 h-full bg-center bg-no-repeat"
                                    style=format!("background-image: url('{}'); background-size: contain;", background_image_url)
                                ></div>

                                <div class="w-1/2 p-12 flex flex-col justify-center text-center bg-black text-white">
                                    <h2 class="mb-4 text-4xl font-semibold">{&build.name}</h2>
                                    <h4 class="mb-6 text-xl font-semibold">{&build.description}</h4>
                                    <button
                                      type="button"
                                      class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
                                    >
                                    <a href={format!{"/builds/{}/{}/{}",&build.name, 0, 1}}>
                                      Learn More
                                    </a>
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            }
        })
    };

    let loading = async_builds.loading();
    let _is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                {move || list_of_builds().unwrap_or_else(|| view! { <div><li>"Loading..."</li></div> })}
            </Suspense>
        </div>
    }
}
