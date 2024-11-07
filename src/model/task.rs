use chrono::prelude::*;
use std::convert::{From, Into};
use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TaskStutus {
    Active,
    Cancelled,
    Completed,
    Deleted,
    Expired,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TaskCategory {
    Work,
    Personal,
    Health,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    uuid: Uuid,
    title: String,
    category: TaskCategory,
    description: String,
    deadline: DateTime<Local>,
    priority: TaskPriority,
    notes: String,
    status: TaskStutus,
}

pub enum TaskOpResult {
    Success,
    Failure(String),
}

impl TryFrom<&str> for TaskPriority {
    type Error = String;
    fn try_from(value: &str) -> Result<TaskPriority, String> {
        match value.trim() {
            "Low" => Ok(TaskPriority::Low),
            "Medium" => Ok(TaskPriority::Medium),
            "High" => Ok(TaskPriority::High),
            _ => Err(format!("Unable to convert {value} to TaskPriority")),
        }
    }
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Medium
    }
}

impl TryFrom<&str> for TaskStutus {
    type Error = String;
    fn try_from(value: &str) -> Result<TaskStutus, String> {
        match value.trim() {
            "Active" => Ok(TaskStutus::Active),
            "Cancelled" => Ok(TaskStutus::Cancelled),
            "Completed" => Ok(TaskStutus::Completed),
            "Deleted" => Ok(TaskStutus::Deleted),
            "Expired" => Ok(TaskStutus::Expired),
            _ => Err(format!("Unable to convert {value} to TaskStutus")),
        }
    }
}

impl Default for TaskStutus {
    fn default() -> Self {
        TaskStutus::Active
    }
}

impl TryFrom<&str> for TaskCategory {
    type Error = String;

    fn try_from(value: &str) -> Result<TaskCategory, String> {
        match value.trim() {
            "Work" => Ok(TaskCategory::Work),
            "Personal" => Ok(TaskCategory::Personal),
            "Health" => Ok(TaskCategory::Health),
            _ => Err(format!("Unable to convert {value} to TaskCategory")),
        }
    }
}

impl Default for TaskCategory {
    fn default() -> Self {
        TaskCategory::Personal
    }
}

impl Task {
    pub fn new(
        category: TaskCategory,
        title: String,
        description: String,
        deadline: DateTime<Local>,
        priority: TaskPriority,
        notes: String,
    ) -> Task {
        Task {
            uuid: Uuid::new_v4(),
            title: title,
            category: category,
            description: description,
            deadline: deadline,
            priority: priority,
            notes: notes,
            status: TaskStutus::Active,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.uuid
    }

    pub fn modify(&self, title: &str) -> Result<TaskOpResult, String> {
        // TODO: request the task manager to modify the title for this task
        Ok(TaskOpResult::Success)
    }

    pub fn update(
        &mut self,
        description: String,
        deadline: DateTime<Local>,
        priority: TaskPriority,
        notes: String,
    ) -> Result<TaskOpResult, String> {
        self.description = description;
        self.deadline = deadline;
        self.priority = priority;
        self.notes = notes;

        Ok(TaskOpResult::Success)
    }

    pub fn cancel(&mut self) -> Result<TaskOpResult, TaskOpResult> {
        // TODO: Validate if this task is ready for cancellation
        self.status = TaskStutus::Cancelled;

        Ok(TaskOpResult::Success)
    }

    pub fn complete(&mut self) -> Result<TaskOpResult, TaskOpResult> {
        // TODO: Validate if this task is ready for completion
        self.status = TaskStutus::Completed;

        Ok(TaskOpResult::Success)
    }

    pub fn delete_by_id(&mut self, id: &Uuid) -> Result<TaskOpResult, TaskOpResult> {
        // TODO: Validate if this task is ready for delete
        self.status = TaskStutus::Deleted;

        Ok(TaskOpResult::Success)
    }

    pub fn delete_by_title(&mut self, title: &str) -> Result<TaskOpResult, TaskOpResult> {
        // TODO: Validate if this task is ready for delete
        self.status = TaskStutus::Deleted;

        Ok(TaskOpResult::Success)
    }

    pub fn erase(&mut self) -> Result<TaskOpResult, TaskOpResult> {
        if self.status == TaskStutus::Deleted {
            // TODO: Validate if this task is ready for erase
            // TODO: Request task manager to delete this task permenently

            Ok(TaskOpResult::Success)
        } else {
            Err(TaskOpResult::Failure("To completely erase the task from the disk, first delete it and then request erase.".into()))
        }
    }

    pub fn expire(&mut self) -> Result<TaskOpResult, TaskOpResult> {
        if self.status == TaskStutus::Completed {
            // TODO: Validate if this task is ready for expiration
            self.status = TaskStutus::Expired;
            Ok(TaskOpResult::Success)
        } else {
            Err(TaskOpResult::Failure(
                "To expire this task it should be completed first.".into(),
            ))
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: Default::default(),
            title: Default::default(),
            category: Default::default(),
            description: Default::default(),
            deadline: Default::default(),
            priority: Default::default(),
            notes: Default::default(),
            status: Default::default(),
        }
    }
}

impl TryFrom<&str> for Task {
    type Error = Vec<String>;
    fn try_from(one_task_str: &str) -> Result<Task, Vec<String>> {
        let mut error_messages: Vec<String> = Vec::new();
        let mut task = Task::default();

        let str_tokens: Vec<&str> = one_task_str.split("^^").collect();

        if str_tokens.len() < 8 {
            error_messages.push("Insufficient fields for parsing into Task".into());
        }
        for (id, token) in str_tokens.iter().enumerate() {
            match id {
                0 => {
                    let parse_result = Uuid::parse_str(*token);
                    match parse_result {
                        Ok(uuid_value) => task.uuid = uuid_value,
                        Err(error) => error_messages.push(error.to_string()),
                    }
                }
                1 => task.title = (*token).trim().to_string(),
                2 => {
                    let parse_result = TaskCategory::try_from(*token);
                    match parse_result {
                        Ok(category_value) => task.category = category_value,
                        Err(error) => error_messages.push(error.to_string()),
                    }
                }
                3 => task.description = (*token).trim().to_string(),
                4 => {
                    let parse_result = (*token).parse::<DateTime<Local>>();
                    match parse_result {
                        Ok(deadline_value) => task.deadline = deadline_value,
                        Err(error) => error_messages.push(error.to_string()),
                    }
                }
                5 => {
                    let parse_result = TaskPriority::try_from(*token);
                    match parse_result {
                        Ok(priority_value) => task.priority = priority_value,
                        Err(error) => error_messages.push(error.to_string()),
                    }
                }
                6 => task.notes = (*token).trim().to_string(),
                7 => {
                    let parse_result = TaskStutus::try_from(*token);
                    match parse_result {
                        Ok(status_value) => task.status = status_value,
                        Err(error) => error_messages.push(error.to_string()),
                    }
                }

                _ => break,
            }
        }
        if error_messages.len() > 0 {
            Err(error_messages)
        } else {
            Ok(task)
        }
    }
}

#[cfg(test)]
mod task_tests {

    #[test]
    pub fn call_hello_world() {
        println!("Hello World");
    }
}
