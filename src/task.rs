use chrono::prelude::*;

pub struct Task {
    category: TaskCategory,
    title: String,
    description: String,
    deadline: DateTime<Local>,
    priority: TaskPriority,
    status: TaskStutus,
}

impl Task {
    pub fn new(
        category: TaskCategory,
        title: String,
        description: String,
        deadline: DateTime<Local>,
        priority: TaskPriority,
    ) -> Task {
        Self {
            category: category,
            title: title,
            description: description,
            deadline: deadline,
            priority: priority,
            status: TaskStutus::Active,
        }
    }
}
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

pub enum TaskStutus {
    Active,
    Cancelled,
    Completed,
    Expired,
}

pub enum TaskCategory {
    Work, 
    Personal,
    Health
}

#[cfg(test)]
mod task_tests {

    #[test]
    pub fn call_hello_world() {
        println!("Hello World");
    }
}
