use rust_task_manager::{errors::TaskError::{DescriptionTooLong, IdInvalid, PriorityInvalid, StatusInvalid, TitleEmpty, TitleTooLong}, tasks::{TaskDescription, TaskId, TaskPriority, TaskStatus, TaskTitle}};

mod tests {
    use super::*;

    // TaskTitle ───────────────────────────────────────────────
    #[test]
    fn title_valid() {
        assert_eq!(TaskTitle::try_from("Hello").unwrap().into_inner(), "Hello");
    }

    #[test]
    fn title_empty() {
        assert_eq!(TaskTitle::try_from("").unwrap_err(), TitleEmpty);
    }

    #[test]
    fn title_too_long() {
        assert_eq!(TaskTitle::try_from("a".repeat(51).as_str()).unwrap_err(), TitleTooLong);
    }

    #[test]
    fn title_from_string_valid() {
        assert_eq!(TaskTitle::try_from("World".to_string()).unwrap().into_inner(), "World");
    }

    // TaskPriority ───────────────────────────────────────────────
    #[test]
    fn priority_valid() {
        assert_eq!(TaskPriority::try_from("low").unwrap(), TaskPriority::Low);
        assert_eq!(TaskPriority::try_from("medium").unwrap(), TaskPriority::Medium);
        assert_eq!(TaskPriority::try_from("high").unwrap(), TaskPriority::High);
    }

    #[test]
    fn priority_invalid() {
        assert_eq!(TaskPriority::try_from("invalid").unwrap_err(), PriorityInvalid);
    }

    #[test]
    fn priority_from_string_valid() {
        assert_eq!(TaskPriority::try_from("high".to_string()).unwrap(), TaskPriority::High);
    }

    // TaskStatus ──────────────────────────────────────────────────
    #[test]
    fn status_valid() {
        assert_eq!(TaskStatus::try_from("new").unwrap(), TaskStatus::New);
        assert_eq!(TaskStatus::try_from("in progress").unwrap(), TaskStatus::InProgress);
        assert_eq!(TaskStatus::try_from("completed").unwrap(), TaskStatus::Completed);
    }

    #[test]
    fn status_invalid() {
        assert_eq!(TaskStatus::try_from("invalid").unwrap_err(), StatusInvalid);
    }

    #[test]
    fn status_from_string_valid() {
        assert_eq!(TaskStatus::try_from("completed".to_string()).unwrap(), TaskStatus::Completed);
    }

    // TaskId ──────────────────────────────────────────────────
    #[test]
    fn taskid_valid() {
        assert_eq!(TaskId::try_from(1).unwrap().into_inner(), 1);
    }

    #[test]
    fn taskid_invalid() {
        assert_eq!(TaskId::try_from(0).unwrap_err(), IdInvalid);
    }

    // TaskDescription ───────────────────────────────────────────────
    #[test]
    fn description_valid() {
        assert_eq!(TaskDescription::try_from("A description").unwrap().into_inner(), "A description");
    }

    #[test]
    fn description_too_long() {
        assert_eq!(TaskDescription::try_from("a".repeat(101).as_str()).unwrap_err(), DescriptionTooLong);
    }

    #[test]
    fn description_from_string_valid() {
        assert_eq!(TaskDescription::try_from("A description".to_string()).unwrap().into_inner(), "A description");
    }
}
