use axum::{http::StatusCode, response::{IntoResponse, Response}};

// Custom Task errors ──────────────────────────────────────────────────
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TaskError {
    #[error("Task ID invalid. Can only be more than 0")]
    IdInvalid,

    #[error("Title cannot be empty")]
    TitleEmpty,

    #[error("Title is too long. Max 50 characters")]
    TitleTooLong,

    #[error("Description is too long. Max 100 characters")]
    DescriptionTooLong,

    #[error("Please enter a valid priority")]
    PriorityInvalid,

    #[error("Please enter a valid status")]
    StatusInvalid,
}

// Custom User errors ──────────────────────────────────────────────────
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum UserError {
    #[error("User ID invalid. Can only be more than 0")]
    IdInvalid,

    #[error("Username cannot be empty")]
    UsernameEmpty,

    #[error("Username is too long. Max 50 characters")]
    UsernameTooLong,

    #[error("Password cannot be empty")]
    PasswordEmpty,

    #[error("Password is too short. Min 8 characters")]
    PasswordTooShort,

    #[error("Invalid credentials")]
    InvalidUsername,
}

// Custom app error ──────────────────────────────────────────────────
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let err = self.0;
        if err.is::<TaskError>() || err.is::<UserError>() {
            (
                StatusCode::BAD_REQUEST,
                format!("Validation error: {}", err),
            ).into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong on our end.",
            ).into_response()
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
