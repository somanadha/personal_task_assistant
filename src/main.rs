use chrono::Local;
mod task;
use task::*;
fn main() {
    let task = Task::new(
        TaskCategory::Personal,
        "String".to_string(),
        "String".to_string(),
        Local::now(),
        TaskPriority::Medium,

    );
}
