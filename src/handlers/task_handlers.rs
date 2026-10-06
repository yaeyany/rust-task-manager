use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use crate::{db::AppDB, errors::AppError, tasks::{Task, TaskDescription, TaskId, TaskPriority, TaskStatus, TaskTitle}};

// Task create struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestTaskCreate {
    title: String,
    description: Option<String>,
}

// Task patch struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestTaskPatch {
    title: String,
    description: Option<String>,
    priority: String,
    status: String,
}

// Task creation validation Json -> TaskTitle and TaskDescription ──────────────────────────────────────────────────
fn validate_task_request(
    request: RequestTaskCreate,
) -> Result<(TaskTitle, Option<TaskDescription>), anyhow::Error> {
    let title = request.title.try_into()?;

    let description = request
        .description
        .map(TaskDescription::try_from)
        .transpose()?;

    Ok((title, description))
}

// Task patch validation Json -> Task struct fields ──────────────────────────────────────────────────
fn validate_patch_request(
    request: RequestTaskPatch,
) -> Result<(TaskTitle, Option<TaskDescription>, TaskPriority, TaskStatus), anyhow::Error> {
    let title = request.title.try_into()?;

    let description = request
        .description
        .map(TaskDescription::try_from)
        .transpose()?;

    let priority = request.priority.try_into()?;
    let status = request.status.try_into()?;

    Ok((title, description, priority, status))
}

// Creating a task ──────────────────────────────────────────────────
pub async fn handler_task_create(
    State(tasks): State<AppDB>,
    Json(request): Json<RequestTaskCreate>,
) -> Result<Json<TaskId>, AppError> {
    let (title, description) = validate_task_request(request)?;

    let id = tasks.add_task(title, description).await?;

    Ok(Json(id))
}

// List all tasks ──────────────────────────────────────────────────
pub async fn handler_task_list(
    State(tasks): State<AppDB>,
) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = tasks.get_tasks(100).await?;

    Ok(Json(tasks))
}

// Patch a task ──────────────────────────────────────────────────
pub async fn handler_task_patch(
    State(tasks): State<AppDB>,
    Path(id): Path<i64>, 
    Json(request): Json<RequestTaskPatch>,
) -> Result<(), AppError> {
    let task_id = id.try_into()?;
    let (title, description, priority, status) = validate_patch_request(request)?;

    tasks.patch_task(task_id, title, description, priority, status).await?;
    Ok(())
}

// Delete a task ──────────────────────────────────────────────────
pub async fn handler_task_delete(
    State(tasks): State<AppDB>,
    Path(id): Path<i64>, 
) -> Result<(), AppError> {
    let task_id = id.try_into()?;
    tasks.delete_task(task_id).await?;
    Ok(())
}
