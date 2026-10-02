use serde::{Deserialize, Serialize};

use crate::{errors::TaskError::{self, *}, helpers::sanitize_string};

// Types ──────────────────────────────────────────────────
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    description: Option<TaskDescription>,
    priority: TaskPriority,
    status: TaskStatus,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct TaskId(i64);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct TaskTitle(String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct TaskDescription(String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    New,
    #[serde(rename = "in progress")] // <-- This tells Serde to output "in progress" with a space
    InProgress,
    Completed,
}

// Traits ──────────────────────────────────────────────────

// TaskId traits ──────────────────────────────────────────────────
impl TryFrom<i64> for TaskId {
    type Error = TaskError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err(IdInvalid)
        } else {
            Ok(TaskId(value))
        }
    }
}

// TaskTitle traits ──────────────────────────────────────────────────
impl TryFrom<&str> for TaskTitle {
    type Error = TaskError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(TitleEmpty)
        } else if value.len() > 50 {
            Err(TitleTooLong)
        } else {
            Ok(TaskTitle(value.to_string()))
        }
    }
}

impl TryFrom<String> for TaskTitle {
    type Error = TaskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

// TaskDescription traits ──────────────────────────────────────────────────
impl TryFrom<&str> for TaskDescription {
    type Error = TaskError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 100 {
            Err(DescriptionTooLong)
        } else {
            Ok(TaskDescription(value.to_string()))
        }
    }
}

impl TryFrom<String> for TaskDescription {
    type Error = TaskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

// TaskPriority traits ──────────────────────────────────────────────────
impl TryFrom<&str> for TaskPriority {
    type Error = TaskError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = sanitize_string(value);
        match value.as_str() {
            "low" => Ok(TaskPriority::Low),
            "medium" => Ok(TaskPriority::Medium),
            "high" => Ok(TaskPriority::High),
            _ => Err(PriorityInvalid),
        }
    }
}

impl TryFrom<String> for TaskPriority {
    type Error = TaskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

// TaskStatus traits ──────────────────────────────────────────────────
impl TryFrom<&str> for TaskStatus {
    type Error = TaskError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = sanitize_string(value);
        match value.as_str() {
            "new" => Ok(TaskStatus::New),
            "in progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            _ => Err(StatusInvalid),
        }
    }
}

impl TryFrom<String> for TaskStatus {
    type Error = TaskError;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }

}

// Methods ──────────────────────────────────────────────────

// TaskTitle methods ──────────────────────────────────────────────────
impl TaskTitle {
    pub fn into_inner(self) -> String {
        self.0
    }
}
// TaskDescription methods ──────────────────────────────────────────────────
impl TaskDescription {
    pub fn into_inner(self) -> String {
        self.0
    }
}
// TaskId methods ──────────────────────────────────────────────────
impl TaskId {
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
// TaskPriority methods ──────────────────────────────────────────────────
impl TaskPriority {
    pub fn into_inner(self) -> String {
        match self {
            TaskPriority::Low => "low".to_string(),
            TaskPriority::Medium => "medium".to_string(),
            TaskPriority::High => "high".to_string(),
        }
    }
}
// TaskStatus methods ──────────────────────────────────────────────────
impl TaskStatus {
    pub fn into_inner(self) -> String {
        match self {
            TaskStatus::New => "new".to_string(),
            TaskStatus::InProgress => "in progress".to_string(),
            TaskStatus::Completed => "completed".to_string(),
        }
    }
}

// Task methods ──────────────────────────────────────────────────
impl Task {

    //Make from parts
    pub fn from_parts(
        id: TaskId,
        title: TaskTitle,
        description: Option<TaskDescription>,
        priority: TaskPriority,
        status: TaskStatus,
    ) -> Self {
        Self {
            id,
            title,
            description,
            priority,
            status,
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::{tasks::*};

    // TaskTitle tests ───────────────────────────────────────────────
    #[test]
    fn test_task_title_valid() {
        let title = TaskTitle::try_from("Hello").unwrap();
        assert_eq!(title.0, "Hello");
    }

    #[test]
    fn test_task_title_empty() {
        let err = TaskTitle::try_from("").unwrap_err();
        assert_eq!(err, TitleEmpty);
    }

    #[test]
    fn test_task_title_too_long() {
        let long = "a".repeat(51);
        let err = TaskTitle::try_from(long.as_str()).unwrap_err();
        assert_eq!(err, TitleTooLong);
    }

    #[test]
    fn test_task_title_from_string_valid() {
        let title = TaskTitle::try_from("World".to_string()).unwrap();
        assert_eq!(title.0, "World");
    }

    // TaskPriority tests ───────────────────────────────────────────────
    #[test]
    fn test_task_priority_valid() {
        let p = TaskPriority::try_from("low").unwrap();
        assert_eq!(p, TaskPriority::Low);

        let p = TaskPriority::try_from("medium").unwrap();
        assert_eq!(p, TaskPriority::Medium);

        let p = TaskPriority::try_from("high").unwrap();
        assert_eq!(p, TaskPriority::High);
    }

    #[test]
    fn test_task_priority_invalid() {
        let err = TaskPriority::try_from("invalid").unwrap_err();
        assert_eq!(err, PriorityInvalid);
    }

    #[test]
    fn test_task_priority_from_string() {
        let p = TaskPriority::try_from("high".to_string()).unwrap();
        assert_eq!(p, TaskPriority::High);
    }

    // TaskStatus tests ──────────────────────────────────────────────────
    #[test]
    fn test_task_status_valid() {
        let s = TaskStatus::try_from("new").unwrap();
        assert_eq!(s, TaskStatus::New);

        let s = TaskStatus::try_from("in progress").unwrap();
        assert_eq!(s, TaskStatus::InProgress);

        let s = TaskStatus::try_from("completed").unwrap();
        assert_eq!(s, TaskStatus::Completed);
    }

    #[test]
    fn test_task_status_invalid() {
        let err = TaskStatus::try_from("invalid").unwrap_err();
        assert_eq!(err, StatusInvalid);
    }

    #[test]
    fn test_task_status_from_string() {
        let s = TaskStatus::try_from("completed".to_string()).unwrap();
        assert_eq!(s, TaskStatus::Completed);
    }
}




