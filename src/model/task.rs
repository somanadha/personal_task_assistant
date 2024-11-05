use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    id: Uuid,
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
            id: Uuid::new_v4(),
            title: title,
            category: category,
            description: description,
            deadline: deadline,
            priority: priority,
            status: TaskStutus::Active,
            notes: notes,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
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

#[cfg(test)]
mod task_tests {

    #[test]
    pub fn call_hello_world() {
        println!("Hello World");
    }
}
