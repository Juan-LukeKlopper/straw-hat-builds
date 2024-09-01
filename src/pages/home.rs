use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to StrawHatBuilds!"</h1>
        <p>"We are builders who want to help you get your projects into production!"</p>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
