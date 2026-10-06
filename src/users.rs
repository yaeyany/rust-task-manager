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

// Tests ──────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use crate::{errors::UserError::*, users::*};

    // UserName ───────────────────────────────────────────────
    #[test]
    fn username_valid() {
        assert_eq!(UserName::try_from("Hello").unwrap().0, "Hello");
    }

    #[test]
    fn username_empty() {
        assert_eq!(UserName::try_from("").unwrap_err(), UsernameEmpty);
    }

    #[test]
    fn username_too_long() {
        assert_eq!(UserName::try_from("a".repeat(51).as_str()).unwrap_err(), UsernameTooLong);
    }

    #[test]
    fn username_from_string_valid() {
        assert_eq!(UserName::try_from("World".to_string()).unwrap().0, "World");
    }

    // UserPassword ───────────────────────────────────────────────
    #[test]
    fn password_valid() {
        assert_eq!(UserPassword::try_from("password123").unwrap().0, "password123");
    }

    #[test]
    fn password_empty() {
        assert_eq!(UserPassword::try_from("").unwrap_err(), PasswordEmpty);
    }

    #[test]
    fn password_too_short() {
        assert_eq!(UserPassword::try_from("short").unwrap_err(), PasswordTooShort);
    }

    #[test]
    fn password_from_string_valid() {
        assert_eq!(UserPassword::try_from("password123".to_string()).unwrap().0, "password123");
    }

    // UserId ──────────────────────────────────────────────────
    #[test]
    fn userid_valid() {
        assert_eq!(UserId::try_from(1).unwrap().0, 1);
    }

    #[test]
    fn userid_invalid() {
        assert_eq!(UserId::try_from(0).unwrap_err(), IdInvalid);
    }
}