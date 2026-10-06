use axum::{Json, extract::{Path, State}};
use serde::Deserialize;
use crate::{db::AppDB, errors::AppError, users::{User, UserId, UserName}};

// User create struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestUserName {
    name: String
}

// User creation validation Json -> UserTitle ──────────────────────────────────────────────────
fn validate_user_request(
    request: RequestUserName,
) -> Result<UserName, anyhow::Error> {
    let name = request.name.try_into()?;
    Ok(name)
}

// Creating a user ──────────────────────────────────────────────────
pub async fn handler_user_create(
    State(users): State<AppDB>,
    Json(request): Json<RequestUserName>,
) -> Result<Json<UserId>, AppError> {
    let name = validate_user_request(request)?;

    let id = users.add_user(name).await?;

    Ok(Json(id))
}

// List all users ──────────────────────────────────────────────────
pub async fn handler_user_list(
    State(users): State<AppDB>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = users.get_users(100).await?;

    Ok(Json(users))
}

// Patch a user ──────────────────────────────────────────────────
pub async fn handler_user_patch(
    State(tasks): State<AppDB>,
    Path(id): Path<i32>, 
    Json(request): Json<RequestUserName>,
) -> Result<(), AppError> {
    let user_id = id.try_into()?;
    let name = validate_user_request(request)?;

    tasks.patch_user(user_id, name).await?;
    Ok(())
}

// Delete a user ──────────────────────────────────────────────────
pub async fn handler_user_delete(
    State(users): State<AppDB>,
    Path(id): Path<i32>, 
) -> Result<(), AppError> {
    let user_id = id.try_into()?;
    users.delete_user(user_id).await?;
    Ok(())
}