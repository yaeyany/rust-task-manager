use axum::{
    Router, http::header, routing::{get, patch},
};

use crate::{db::AppDB, handlers::{*, task_handlers::*, user_handlers::*}};

// Main router ──────────────────────────────────────────────────
pub fn router(tasks: AppDB) -> Router {
    Router::new()
        
        // Style css ──────────────────────────────────────────────────
        .route("/static/style.css", get(|| async {
        (
            [(header::CONTENT_TYPE, "text/css")],
            include_str!("../static/style.css"),
        )}))
       .route("/static/password-input.js", get(|| async {
            (
                [(header::CONTENT_TYPE, "application/javascript")],
                include_str!("../static/password-input.js"),
            )
        }))

        // Home page ──────────────────────────────────────────────────
        .route("/login", 
            get(|| html_handler("templates/login.html"))
            .post(handler_login),
        )   

        // Tasks nest ──────────────────────────────────────────────────
        .nest("/task", task_router())

        // User nest ──────────────────────────────────────────────────
        .nest("/user", user_router())
        
        // Api nest ──────────────────────────────────────────────────
        .nest("/api", api_router())
        
        // Fallback url ──────────────────────────────────────────────────
        .fallback(get(redirect_to_login))
        
        // State ──────────────────────────────────────────────────
        .with_state(tasks)
}

// API router ──────────────────────────────────────────────────
pub fn api_router() -> Router<AppDB> {
    Router::new()

    .route("/task/list", get(handler_task_list))
    .route("/user/list", get(handler_user_list))
}

// Tasks router ──────────────────────────────────────────────────
pub fn task_router() -> Router<AppDB> {
    Router::new()
        
        .route("/list", get(|| { html_handler("templates/task_list.html")}))
        .route(
            "/create",
            get(|| { html_handler("templates/task_create.html")})
                .post(handler_task_create),
        )
        .route("/{id}", 
            patch(handler_task_patch)
            .delete(handler_task_delete)
            .get(redirect_to_login))
}

// User router ──────────────────────────────────────────────────
pub fn user_router() -> Router<AppDB> {
    Router::new()
       
        .route("/list", get(|| { html_handler("templates/user_list.html")}))
        .route("/create",
            get(|| { html_handler("templates/user_create.html")})
                .post(handler_user_create),
        )
        .route("/{id}", 
            patch(handler_user_name_patch)
            .delete(handler_user_delete)
            .get(redirect_to_login)
        )
}
