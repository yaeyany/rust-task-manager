use axum::{
    Router, http::header, routing::{get, patch},
};

use crate::database::TasksDB;
use crate::handlers::*;

pub fn router(tasks: TasksDB) -> Router {
    Router::new()
        .route("/static/style.css", get(|| async {
        (
            [(header::CONTENT_TYPE, "text/css")],
            include_str!("../static/style.css"),
        )}))
        .nest("/task", task_router())
        .route("/api/task/list", get(handler_task_list))
        .fallback(get(redirect_to_home))
        .with_state(tasks)
}

pub fn task_router() -> Router<TasksDB> {
    Router::new()
        .route("/list", get(|| { html_handler("templates/task_list.html")}))
        .route(
            "/create",
            get(|| { html_handler("templates/task_create.html")})
                .post(handler_task_create),
        )
        .route("/{id}", 
            patch(handler_task_patch)
            .get(redirect_to_home)
            .delete(handler_task_delete))
}