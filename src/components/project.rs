use leptos::*;
use leptos_dom::logging::console_log;
use leptos_router::*;
use serde::{Deserialize, Serialize};


use web_sys::{js_sys, wasm_bindgen::JsValue};
use wasm_bindgen::prelude::*;
use web_sys::{window, console};
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    pub name: String,
    pub text: String,
    pub num: u8,
    pub final_chapter: bool,
    pub final_section: bool,
}

#[derive(Params, PartialEq)]
struct ProjectParams {
    project_name: String,
    section_num: u8,
    chapter_num: u8,
}

async fn get_keplr_address() -> Result<String, JsValue> {
    let window = window().unwrap();

    // Check if Keplr is available in the browser
    if let Some(keplr) = js_sys::Reflect::get(&window, &JsValue::from_str("keplr")).ok() {
        if keplr.is_undefined() {
            return Err(JsValue::from_str("Keplr extension not found!"));
        }

        // Enable Keplr for the chain
        let chain_id = "secret-4"; // Replace with your chain ID
        let enable_method = js_sys::Reflect::get(&keplr, &JsValue::from_str("enable")).unwrap();
        let enable_promise: Promise = enable_method
            .dyn_into::<js_sys::Function>()
            .unwrap()
            .call1(&keplr, &JsValue::from_str(chain_id))
            .unwrap()
            .into();
        JsFuture::from(enable_promise).await?;

        // Get Keplr key (which contains the user's address)
        let get_key_method = js_sys::Reflect::get(&keplr, &JsValue::from_str("getKey")).unwrap();
        let key_promise: Promise = get_key_method
            .dyn_into::<js_sys::Function>()
            .unwrap()
            .call1(&keplr, &JsValue::from_str(chain_id))
            .unwrap()
            .into();
        let key_jsvalue = JsFuture::from(key_promise).await?;

        // Extract the Bech32 address from the key object
        let bech32_address = js_sys::Reflect::get(&key_jsvalue, &JsValue::from_str("bech32Address")).unwrap();
        let address = bech32_address.as_string().unwrap_or_default();

        Ok(address)
    } else {
        Err(JsValue::from_str("Keplr extension not available!"))
    }
}

#[server(Test, "/api")]
pub async fn test(address: String, build: String) -> Result<(), ServerFnError> {
    print!("TEST");
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

#[server(GetProject, "/project")]
pub async fn get_project_chapter(
    project_name: String,
    section_num: u8,
    chapter_num: u8,
) -> Result<Chapter, ServerFnError> {
    use pulldown_cmark::{Options, Parser};
    use std::fs;
    use std::path::PathBuf;

    let mut final_c = true;
    let mut final_s = true;
    let mut chapter_text = String::from("");
    let mut file_name = String::from("");

    let current_chapter_path = format!(
        "./content/{}/section_{}/Chapter{}/",
        project_name, section_num, chapter_num
    );
    let next_chapter_path = format!(
        "./content/{}/section_{}/Chapter{}",
        project_name,
        section_num,
        chapter_num + 1
    );
    let next_section_path = format!(
        "./content/{}/section_{}/Chapter{}",
        project_name,
        section_num + 1,
        1
    );

    // Search for any file in the current chapter directory
    if let Ok(entries) = fs::read_dir(&current_chapter_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                    file_name = fname.to_string();
                    if let Ok(content) = fs::read_to_string(&path) {
                        chapter_text = content;
                    }
                }
                break; // Stop after the first file is found
            }
        }
    }


    // Check for the existence of the next chapter and section
    if let Ok(_) = fs::read_dir(next_chapter_path) {
        final_c = false;
    }

    if let Ok(_) = fs::read_dir(next_section_path) {
        final_s = false;
    }

    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&chapter_text, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Return the chapter details
    let res = Chapter {
        name: file_name,
        text: html_output,
        num: chapter_num,
        final_chapter: final_c,
        final_section: final_s,
    };

    Ok(res)
}


#[component]
pub fn Project_output() -> impl IntoView {
        // Use reactive signals that will cause the resource to refetch on parameter change
    let (chapter, _set_posts) = create_signal(Chapter {
        name: String::from(""),
        text: String::from(""),
        num: 0,
        final_section: true,
        final_chapter: true,
    });


    let params = use_params::<ProjectParams>();

    let project_name = create_memo(move |_| {
    params.with(|params| {
        params.as_ref()
            .map(|params| params.project_name.clone())
            .unwrap_or_default()
    })
});

let section_num = create_memo(move |_| {
    params.with(|params| {
        params.as_ref()
            .map(|params| params.section_num)
            .unwrap_or_default()
    })
});

let chapter_num = create_memo(move |_| {
    params.with(|params| {
        params.as_ref()
            .map(|params| params.chapter_num)
            .unwrap_or_default()
    })
});

    // Refetch data based on reactive params
    let async_chapters = create_resource(
        move || (project_name(), section_num(), chapter_num()),
        move |(name, section, num)| async move {
            get_project_chapter(name, section, num).await.unwrap()
        },
    );




    let done_handler = move |_| {
    let project_name = project_name();
    if cfg!(target_arch = "wasm32") {
        spawn_local(async move {
            match get_keplr_address().await {
                Ok(address) => {
                    console::log_1(&JsValue::from_str(&format!("Keplr address: {}", address)));
                    test(address, project_name).await;

                }
                Err(e) => {
                    console::error_1(&e);
                }
            }
        });
    }
};
    

























    let async_chapter = move || {
        async_chapters
            .get()
            .map(|chapter| {
                view! {
                    <div class="container mx-auto px-5 py-2 lg:px-32 lg:pt-12 space-y-6">
                        <h2 class="mb-4 text-4xl font-semibold">{&chapter.name}</h2>
                        <div class="project" inner_html=&chapter.text></div>
                            {if chapter.final_chapter != true {
                                view!{
                                    <button
                                        type="button"
                                        class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
                                    >
                                    <a href={format!{"/builds/{}/{}/{}",&project_name(), &section_num(), &chapter.num+1}}>
                                        Next Chapter
                                    </a>
                                    </button>
                                }
                            }else if chapter.final_section != true {
                                    view!{
                                        <button
                                            type="button"
                                            class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
                                        >
                                            <a href={format!{"/builds/{}/{}/{}",&project_name(), &section_num()+1, 1}}>
                                                Next Section
                                            </a>
                                        </button>
                                    }
                            }else {
                                    view!{
                                        <button
                                            type="button"
                                            class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
                                            on:click=done_handler
                                        >
                                            Claim NFT!
                                        </button>
                                    }

                            }} 
                    </div>
                }
            })
            .unwrap_or_else(|| view! { <div>"Chapter not found."</div> })
    };

    view! {
        <div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                {move || async_chapter()}
            </Suspense>
        </div>
    }
}

