use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::errors::UserError;

// User types ──────────────────────────────────────────────────
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct User {
    id: UserId,
    username: UserName,
    created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct UserId(i32);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct UserName(String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct UserPassword(String);

// UserId traits ──────────────────────────────────────────────────
impl TryFrom<i32> for UserId {
    type Error = UserError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err(UserError::IdInvalid)
        } else {
            Ok(UserId(value))
        }
    }
}

// UserName traits ──────────────────────────────────────────────────
impl TryFrom<&str> for UserName {
    type Error = UserError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(UserError::UsernameEmpty)
        } else if value.len() > 50 {
            Err(UserError::UsernameTooLong)
        } else {
            Ok(UserName(value.to_string()))
        }
    }
}

impl TryFrom<String> for UserName {
    type Error = UserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

// UserPassword traits ──────────────────────────────────────────────────
impl TryFrom<&str> for UserPassword {
    type Error = UserError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(UserError::PasswordEmpty)
        } else if value.len() < 8 {
            Err(UserError::PasswordTooShort)
        } else {
            Ok(UserPassword(value.to_string()))
        }
    }
}

impl TryFrom<String> for UserPassword {
    type Error = UserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

// UserId methods ──────────────────────────────────────────────────
impl UserId {
    pub fn into_inner(self) -> i32 {
        self.0
    }
}

// UserName methods ──────────────────────────────────────────────────
impl UserName {
    pub fn into_inner(self) -> String {
        self.0
    }
}

// UserPassword methods ──────────────────────────────────────────────────
impl UserPassword {
    pub fn into_inner(self) -> String {
        self.0
    }
}

// User methods ──────────────────────────────────────────────────
impl User {

    //Make from parts
    pub fn from_parts(
        id: UserId,
        username: UserName,
        created_at: DateTime<Utc>
    ) -> Self {
        Self {
            id,
            username,
            created_at
        }
    }
}
