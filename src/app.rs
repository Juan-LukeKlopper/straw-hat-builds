use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Pages
use crate::pages::about::*;
use crate::pages::home::*;

// components
use crate::components::footer::*;
use crate::components::nav::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/straw-hat-builds.css"/>

        // sets the document title
        <Title text="Strawhat builds"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Header/>
            <main>
                <Routes>
                    <Route path="/" view=|| view ! {<Home/>}/>
                    <Route path="/builds" view=|| view ! {<Home/>}/>
                    <Route path="/about" view=|| view ! {<About/>}/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
