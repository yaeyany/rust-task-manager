use rust_task_manager::{errors::UserError::{UsernameEmpty, UsernameTooLong, PasswordEmpty, PasswordTooShort, IdInvalid}, users::{UserId, UserName, UserPassword}};

mod tests {
    use super::*;

    // UserName ───────────────────────────────────────────────
    #[test]
    fn username_valid() {
        assert_eq!(UserName::try_from("Hello").unwrap().into_inner(), "Hello");
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
        assert_eq!(UserName::try_from("World".to_string()).unwrap().into_inner(), "World");
    }

    // UserPassword ───────────────────────────────────────────────
    #[test]
    fn password_valid() {
        assert_eq!(UserPassword::try_from("password123").unwrap().into_inner(), "password123");
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
        assert_eq!(UserPassword::try_from("password123".to_string()).unwrap().into_inner(), "password123");
    }

    // UserId ──────────────────────────────────────────────────
    #[test]
    fn userid_valid() {
        assert_eq!(UserId::try_from(1).unwrap().into_inner(), 1);
    }

    #[test]
    fn userid_invalid() {
        assert_eq!(UserId::try_from(0).unwrap_err(), IdInvalid);
    }
}
