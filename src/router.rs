use axum::{
    Router, http::header, routing::{get, patch},
};

use crate::database::TasksDB;
use crate::handlers::*;

// Main router ──────────────────────────────────────────────────
pub fn router(tasks: TasksDB) -> Router {
    Router::new()
        
        // Style css ──────────────────────────────────────────────────
        .route("/static/style.css", get(|| async {
        (
            [(header::CONTENT_TYPE, "text/css")],
            include_str!("../static/style.css"),
        )}))
        
        // Tasks nest ──────────────────────────────────────────────────
        .nest("/task", task_router())
        
        // Api nest ──────────────────────────────────────────────────
        .route("/api/task/list", get(handler_task_list))
        
        // Fallback url ──────────────────────────────────────────────────
        .fallback(get(redirect_to_home))
        
        // State ──────────────────────────────────────────────────
        .with_state(tasks)
}

// Tasks router ──────────────────────────────────────────────────
pub fn task_router() -> Router<TasksDB> {
    Router::new()
        
        // Tasks list ──────────────────────────────────────────────────
        .route("/list", get(|| { html_handler("templates/task_list.html")}))
        
        // Task creation ──────────────────────────────────────────────────
        .route(
            "/create",
            get(|| { html_handler("templates/task_create.html")})
                .post(handler_task_create),
        )
        
        // Task patching and deletion, with invalid url detection ──────────────────────────────────────────────────
        .route("/{id}", 
            patch(handler_task_patch)
            .delete(handler_task_delete)

            // Redirect on invalid url ──────────────────────────────────────────────────
            .get(redirect_to_home))
}