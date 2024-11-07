pub mod file;
pub mod task;

use file::FileLoadHandler;
use task::{Task, TaskCategory};

use lazy_static::lazy_static;
use std::{collections::HashMap,  sync::Mutex};

pub struct TaskManager {
    tasks: Option<HashMap<TaskCategory, Vec<Task>>>,
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

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            load_source: LoadSource::FilesDirectoryPath(".".into()),
        }
    }
}

pub struct LoadResults {
    tasks: HashMap<TaskCategory, Vec<Task>>,
}

impl LoadResults {
    pub fn new(tasks: HashMap<TaskCategory, Vec<Task>>) -> Self {
        LoadResults { tasks: tasks }
    }
}

pub trait LoadHandler {
    fn load_tasks(&self) -> Result<LoadResults, String>;
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { tasks: None }
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
                    }
                    Err(error) => return Err(error),
                }
            }
            value => return Err("{value} : Implementation Not Available".into()),
        }
        Ok(())
    }

    pub fn add_task(mut self, task : Task) -> Result<(), String> {
        if (self.tasks != None) {
            self.tasks.unwrap().entry(task.category())
            .or_insert_with(Vec::new)
            .push(task);
        } else {
            return Err("Call TaskManager.load_tasks first before adding a task".into());
        }

        Ok(())
    }
}
