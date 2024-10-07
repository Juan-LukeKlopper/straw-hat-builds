use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    pub name: String,
    pub text: String,
}

#[derive(Params, PartialEq)]
struct ProjectParams {
    project_name: String,
    section_num: u8,
    chapter_num: u8,
}

#[server(GetProject, "/project")]
pub async fn get_project_chapter(
    project_name: String,
    section_num: u8,
    chapter_num: u8,
) -> Result<Chapter, ServerFnError> {
    println!("values: {:?}, {:?}", project_name, chapter_num);
    use pulldown_cmark::{Options, Parser};
    use std::fs;

    let content = format!(
        "./content/{}/section_{}/Chapter{}/index.md",
        project_name, section_num, chapter_num
    ); // Adjust this to your actual content directory path

    let mut chapter_text = String::from("");
    if let Ok(entry) = fs::read_to_string(content) {
        println!("entry: {:?}", entry);
        chapter_text = entry;
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&chapter_text, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    println!("html_output: {:?}", html_output);
    let temp: Chapter = Chapter {
        name: project_name,
        text: html_output,
    };

    Ok(temp)
}

#[component]
pub fn Project_output() -> impl IntoView {
    let params = use_params::<ProjectParams>();

    let project_name = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.project_name.clone())
                .unwrap_or_default()
        })
    };

    let section_num = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.section_num.clone())
                .unwrap_or_default()
        })
    };

    let chapter_num = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.chapter_num.clone())
                .unwrap_or_default()
        })
    };
    let (chapter, _set_posts) = create_signal(Chapter {
        name: String::from(""),
        text: String::from(""),
    });

    let async_chapters = create_resource(
        // the first is the "source signal"
        chapter,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        move |_value| {
            let name = project_name.clone();
            let section = section_num.clone();
            let num = chapter_num.clone();

            async move { get_project_chapter(name(), section(), num()).await.unwrap() }
        },
    );

    let async_chapter = move || {
        async_chapters
            .get()
            .map(|chapter| {
                view! {
                    <div class="container mx-auto px-5 py-2 lg:px-32 lg:pt-12 space-y-6">
                        <h2 class="mb-4 text-4xl font-semibold">{&chapter.name}</h2>
                        <div class="project" inner_html=&chapter.text></div>
                    </div>
                }
            })
            .unwrap_or_else(|| view! { <div>"Chapter not found."</div> })
    };

    //  let async_chapter = move || {
    //    async_chaptera.get().map(|projects| {
    //    view! {
    //               <div class="container mx-auto px-5 py-2 lg:px-32 lg:pt-12 space-y-6">
    //                      {async_chapters}
    //                {projects.iter().map(|project| {
    //                    let background_image_url = format!("/backgrounds/{}.svg", project.name.replace(" ", "%20"));

    //                  view! {
    //                      <div class="flex h-[400px] overflow-hidden rounded-lg bg-gray-100">
    //                          <div
    //                              class="w-1/2 h-full bg-center bg-no-repeat"
    //                              style=format!("background-image: url('{}'); background-size: contain;", background_image_url)
    //                          ></div>

    //                        <div class="w-1/2 p-12 flex flex-col justify-center text-center bg-black text-white">
    //                            <h2 class="mb-4 text-4xl font-semibold">{&project.name}</h2>
    //                            <h4 class="mb-6 text-xl font-semibold">{&project.description}</h4>
    //                            <button
    //                              type="button"
    //                              class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
    //                            >
    //                              Learn More
    //                            </button>
    //                        </div>
    //                    </div>
    //                }
    //            }).collect::<Vec<_>>()}
    //        </div>
    //         }
    //})
    //};

    //let loading = async_projects.loading();
    //let _is_loading = move || if loading() { "Loading..." } else { "Idle." };

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
