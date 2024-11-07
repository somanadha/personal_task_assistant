use std::{collections::HashMap, fs};

use crate::model::{LoadHandler, LoadResults, Task};

pub struct FileLoadHandler {
    file_path: String,
}

impl LoadHandler for FileLoadHandler {
    fn load_tasks(&self) -> Result<LoadResults, String> {
        self.load_tasks_from_file(&self.file_path)
    }
}

impl FileLoadHandler {
    pub const TASK_FILE_NAME: &str = "//pta.tsk";
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
                let task_vector: Vec<Task> = task_data_string
                    .lines()
                    .filter_map(|one_line| match Task::try_from(one_line) {
                        Ok(task) => Some(task),
                        Err(_error) => None, // This needs to be logged,
                    })
                    .collect();

                for task in task_vector {
                    tasks
                        .entry(task.category())
                        .or_insert_with(Vec::new)
                        .push(task);
                }
                Ok(LoadResults::new(tasks))
            }

            Err(error) => Err(error.to_string()),
        }
    }
}
