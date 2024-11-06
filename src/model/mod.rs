pub mod file;
pub mod task;

use task::{Task, TaskCategory};

use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};

pub struct TaskManager {
    tasks: HashMap<TaskCategory, Vec<Task>>,
}

pub enum LoadSource {
    FilesDirectoryPath(String),
    Database(String),
}

pub struct LoadOptions {
    load_cancelled: bool,
    load_expired: bool,
    load_completed: bool,
    load_deleted: bool,
    load_source: LoadSource,
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
            load_source: LoadSource::FilesDirectoryPath(".".into()),
        }
    }
}

pub struct LoadResults {
    active: Option<u32>,
    cancelled: Option<u32>,
    expired: Option<u32>,
    completed: Option<u32>,
    deleted: Option<u32>,
    tasks: HashMap<TaskCategory, Vec<Task>>,
}

impl LoadResults {
    pub fn new() -> Self {
        LoadResults {
            active: None,
            cancelled: None,
            expired: None,
            completed: None,
            deleted: None,
            tasks: HashMap::new(),
        }
    }
}

pub trait LoadHandler {
    fn new(options: LoadOptions) -> Result<Self, String>
    where
        Self: Sized;
    fn load_tasks(&self) -> Result<LoadResults, String>;
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
