use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

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

#[server(GetProject, "/project")]
pub async fn get_project_chapter(
    project_name: String,
    section_num: u8,
    chapter_num: u8,
) -> Result<Chapter, ServerFnError> {
    println!("values: {:?}, {:?}", project_name, chapter_num);
    use pulldown_cmark::{Options, Parser};
    use std::fs;

    let mut final_c = true;
    let mut final_s = true;
    let current_chapter_path = format!(
        "./content/{}/section_{}/Chapter{}/index.md",
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

    let mut chapter_text = String::from("");
    if let Ok(entry) = fs::read_to_string(current_chapter_path) {
        println!("entry: {:?}", entry);
        chapter_text = entry;
    }

    if let Ok(next) = fs::read_dir(next_chapter_path) {
        println!("There is a next chapter");
        final_c = false;
    }

    if let Ok(next_section) = fs::read_dir(next_section_path) {
        println!("There is a next section!");
        final_s = false;
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&chapter_text, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    println!("html_output: {:?}", html_output);
    let res = Chapter {
        name: project_name,
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

    

    let async_chapter = move || {
        async_chapters
            .get()
            .map(|chapter| {
                view! {
                    <div class="container mx-auto px-5 py-2 lg:px-32 lg:pt-12 space-y-6">
                        <h2 class="mb-4 text-4xl font-semibold">{&chapter.name}</h2>
                        <div class="project" inner_html=&chapter.text></div>
                    {if chapter.final_section == true {
                        view!{
                        <button
                                      type="button"
                                      class="inline-block rounded border-2 border-neutral-50 px-6 pb-[6px] pt-2 text-xs font-medium uppercase leading-normal text-neutral-50 transition duration-150 ease-in-out hover:border-neutral-300 hover:text-neutral-200 focus:border-neutral-300 focus:text-neutral-200 focus:outline-none focus:ring-0 active:border-neutral-300 active:text-neutral-200 dark:hover:bg-neutral-600 dark:focus:bg-neutral-600"
                                    >
                                    <a href={format!{"/builds/{}/{}/{}",&chapter.name, 0, &chapter.num+1}}>
                                      Learn More
                                    </a>
                                    </button>
                    }}else{
                            view!{<button>Next Section</button>}
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

