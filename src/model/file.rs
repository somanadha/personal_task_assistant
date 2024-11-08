use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use uuid::Uuid;

use crate::model::{LoadHandler, LoadResults, Task};

pub struct FileLoadHandler {
    file_path: String,
}

impl LoadHandler for FileLoadHandler {
    fn load_tasks(&self) -> Result<LoadResults, String> {
        self.load_tasks_from_file(&self.file_path)
    }

    fn save_tasks(&self, tasks: &[Task]) -> Result<(), String> {
        todo!()
    }
}

impl FileLoadHandler {
    pub const TASK_FILE_NAME: &'static str = "//pta.tsk";
    pub fn new(path: String) -> Self {
        FileLoadHandler { file_path: path }
    }

    fn load_tasks_from_file(&self, files_directory_path: &String) -> Result<LoadResults, String> {
        let file_name = format!(
            "{}{}",
            files_directory_path,
            FileLoadHandler::TASK_FILE_NAME
        );

        match fs::read_to_string(file_name) {
            Ok(task_data_string) => {
                let mut tasks = HashMap::new();
                let uuid_task_hash_map: HashMap<Uuid, Task> = task_data_string
                    .lines()
                    .filter_map(|one_line| match Task::try_from(one_line) {
                        Ok(task) => Some((task.uuid(), task)),
                        Err(_error) => None, // This needs to be logged,
                    })
                    .collect();

                for (uuid, task) in uuid_task_hash_map {
                    tasks
                        .entry(task.category())
                        .or_insert_with(HashMap::new)
                        .insert(uuid, task);
                }
                Ok(LoadResults::new(tasks))
            }
            Err(error) => Err(error.to_string()),
        }
    }

    fn save_tasks_to_file(&self, tasks: &[Task]) -> Result<(), std::io::Error> {
        let file_name = format!("{}{}", self.file_path, FileLoadHandler::TASK_FILE_NAME);

        let mut file = File::create(file_name)?;
        for task in tasks {
            writeln!(file, "{}", task.into())?;
        }

        Ok(())
    }
}
