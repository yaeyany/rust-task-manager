use axum::response::{Html, Redirect};

pub mod task_handlers;
pub mod user_handlers;

// Checking for an html file ──────────────────────────────────────────────────
pub async fn html_handler(path: &str) -> Html<String> {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => Html(content),
        Err(_) => Html(format!(
            "<h1>500 Internal Server Error</h1><p>Critical error: HTML file '<strong>{}</strong>' not found on disk.</p>",
            path
        )),
    }
}
// Redirect to home ──────────────────────────────────────────────────
pub async fn redirect_to_home() -> Redirect {
    Redirect::temporary("/task/create")
}