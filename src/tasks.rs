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
    #[serde(rename = "in progress")] 
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
            "in_progress" | "in progress" => Ok(TaskStatus::InProgress),
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
