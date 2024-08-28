use leptos::*;
use pulldown_cmark::{Parser, Options};
use ammonia::Builder;

fn main() {

    let markdown_input = "Hello world, this is a *very simple* example.";
    let options = Options::empty();
    let parser = Parser::new_ext(markdown_input, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let sanitized_html = Builder::default().clean(&html_output).to_string();
    
    mount_to_body(move || view! { 
        <div inner_html=sanitized_html />
    });
}