use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::{Path, State}};
use serde::{Deserialize, Serialize};
use crate::{db::AppDB, errors::{AppError, UserError}, users::*};

// User name and password request struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestUserPass {
    name: String,
    password: String,
}

// User name request struct ──────────────────────────────────────────────────
#[derive(Deserialize)]
pub struct RequestUserName {
    name: String,
}

// Login response struct ──────────────────────────────────────────────────
#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
}

// User creation validation Json -> Rust types ──────────────────────────────────────────────────
fn validate_user_pass_request(
    request: RequestUserPass,
) -> Result<(UserName, UserPassword), anyhow::Error> {
    let name = request.name.try_into()?;
    let password = request.password.try_into()?;
    Ok((name, password))
}

// User name validation Json -> Rust types ──────────────────────────────────────────────────
fn validate_user_name_request(
    request: RequestUserName,
) -> Result<UserName, anyhow::Error> {
    let name = request.name.try_into()?;
    Ok(name)
}

// Creating a user ──────────────────────────────────────────────────
pub async fn handler_user_create(
    State(users): State<AppDB>,
    Json(request): Json<RequestUserPass>,
) -> Result<Json<UserId>, AppError> {
    let (name, password) = validate_user_pass_request(request)?;

    let id = users.add_user(name, password).await?;

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
pub async fn handler_user_name_patch(
    State(users): State<AppDB>,
    Path(id): Path<i32>, 
    Json(request): Json<RequestUserName>,
) -> Result<(), AppError> {
    let user_id = id.try_into()?;
    let name = validate_user_name_request(request)?;

    users.patch_user_name(user_id, name).await?;
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

// Handle user login authentication ──────────────────────────────────────────────────
pub async fn handler_login(
    State(users): State<AppDB>,
    Json(request): Json<RequestUserPass>,    
) -> Result<Json<LoginResponse>, AppError> {
    let (name, password) = validate_user_pass_request(request)?;
    let password_hash = users
        .get_user_password_hash(&name.into_inner())
        .await?
        .ok_or(UserError::InvalidUsername)?;

    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default().verify_password(&password.into_inner().as_bytes(), &parsed_hash)?;

    Ok(Json(LoginResponse {
        success: true,
    }))
}