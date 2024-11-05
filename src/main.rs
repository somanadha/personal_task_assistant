mod controller;
mod model;
mod view;

use chrono::Local;
use model::task::*;

//use task::*;
fn main() {
    let task = Task::new(
        TaskCategory::Personal,
        "String".to_string(),
        "String".to_string(),
        Local::now(),
        TaskPriority::Medium,
        String::from(""),
    );

    let task_manager = model::TaskManager::get_instance();
}
