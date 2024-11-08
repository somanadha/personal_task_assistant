mod file;
pub mod task;

use file::FileLoadHandler;
use task::{Task, TaskCategory};

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    sync::{Arc, Mutex},
};
use uuid::Uuid;

pub struct TaskManager {
    tasks: Option<HashMap<TaskCategory, HashMap<Uuid, Task>>>,
    handler: Option<Arc<Mutex<dyn LoadHandler + Send>>>,
}

pub enum LoadSource {
    FilesDirectoryPath(String),
    Database(String),
}

pub struct LoadOptions {
    load_source: LoadSource,
}

impl LoadOptions {
    pub fn new(source: LoadSource) -> Self {
        LoadOptions {
            load_source: source,
        }
    }
}

impl Display for LoadSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadSource::FilesDirectoryPath(path) => write!(f, "Files Directory Path: {}", path),
            LoadSource::Database(db) => write!(f, "Database: {}", db),
        }
    }
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            load_source: LoadSource::FilesDirectoryPath(".".into()),
        }
    }
}

struct LoadResults {
    tasks: HashMap<TaskCategory, HashMap<Uuid, Task>>,
}

impl LoadResults {
    pub fn new(tasks: HashMap<TaskCategory, HashMap<Uuid, Task>>) -> Self {
        LoadResults { tasks: tasks }
    }
}

trait LoadHandler {
    fn load_tasks(&self) -> Result<LoadResults, String>;
    fn save_tasks(&self, tasks: &[Task]) -> Result<(), String>;
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: None,
            handler: None,
        }
    }

    pub fn get_instance() -> &'static Mutex<TaskManager> {
        lazy_static! {
            static ref INSTANCE: Mutex<TaskManager> = Mutex::new(TaskManager::new());
        }
        &INSTANCE
    }

    pub fn load_tasks(mut self, options: LoadOptions) -> Result<(), String> {
        match options.load_source {
            LoadSource::FilesDirectoryPath(path) => {
                let handler = FileLoadHandler::new(path);
                match handler.load_tasks() {
                    Ok(results) => {
                        self.tasks = Some(results.tasks);
                        self.handler = Some(Arc::new(Mutex::new(handler)));
                    }
                    Err(error) => return Err(error),
                }
            }
            value => return Err(format!("{value} : Implementation Not Available")),
        }
        Ok(())
    }

    pub fn add_task(mut self, task: Task) -> Result<(), String> {
        if self.tasks != None {
            self.tasks
                .unwrap()
                .entry(task.category())
                .or_insert_with(HashMap::new)
                .insert(task.uuid(), task);
        } else {
            return Err("Call TaskManager.load_tasks first before adding a task".into());
        }

        Ok(())
    }
}
