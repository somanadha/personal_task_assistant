pub mod file;
pub mod task;

use file::FileLoadHandler;
use task::{Task, TaskCategory};

use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub struct TaskManager {
    tasks: HashMap<TaskCategory, Vec<Task>>,
}

pub struct LoadOptions {
    load_cancelled: bool,
    load_expired: bool,
    load_completed: bool,
    load_deleted: bool,
}

impl LoadOptions {
    pub fn new() -> Self {
        LoadOptions::default()
    }

    pub fn load_cancelled(&mut self, load_cancelled: bool) {
        self.load_cancelled = load_cancelled;
    }
    pub fn load_expired(&mut self, load_expired: bool) {
        self.load_expired = load_expired;
    }
    pub fn load_completed(&mut self, load_completed: bool) {
        self.load_completed = load_completed;
    }
    pub fn load_deleted(&mut self, load_deleted: bool) {
        self.load_deleted = load_deleted;
    }
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            load_cancelled: Default::default(),
            load_expired: Default::default(),
            load_completed: Default::default(),
            load_deleted: Default::default(),
        }
    }
}

pub enum LoadResult<'a> {
    Success(&'a dyn LoadHandler),
    Failure(String),
}

pub trait LoadHandler {
    fn load_tasks(options: LoadOptions) -> Result<Self, String>;
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
        }
    }
    pub fn get_instance() -> &'static Mutex<TaskManager> {
        lazy_static! {
            static ref INSTANCE: Mutex<TaskManager> = Mutex::new(TaskManager::new());
        }
        &INSTANCE
    }
}
impl Default for LoadHandler {
    fn default() -> Self {
        FileLoadHandler::load_tasks(LoadOptions::default())
    }
}
