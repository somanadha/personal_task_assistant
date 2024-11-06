use std::{fs, io::Error};

use crate::{
    model::{LoadHandler, LoadOptions, LoadResults, LoadSource},
    Task,
};

pub struct FileLoadHandler {
    load_options: LoadOptions,
}

impl LoadHandler for FileLoadHandler {
    fn new(options: LoadOptions) -> Result<Self, String>
    where
        Self: Sized,
    {
        match options.load_source {
            LoadSource::FilesDirectoryPath(_) => Ok({
                FileLoadHandler {
                    load_options: options,
                }
            }),
            _ => {
                Err("For \"FileLoadHandler\", LoadSource should be \"FilesDirectoryPath\".".into())
            }
        }
    }

    fn load_tasks(&self) -> Result<LoadResults, String> {
        let files_directory_path = match &self.load_options.load_source {
            LoadSource::FilesDirectoryPath(path) => path,
            _ => &".".into(),
        };

        self.load_tasks_from_files(files_directory_path)
    }
}

impl FileLoadHandler {
    pub const CANCELLED_FILE_NAME: &str = "//cancelled.tsk";
    pub const EXPIRED_FILE_NAME: &str = "//expired.tsk";
    pub const COMPLETED_FILE_NAME: &str = "//completed.tsk";
    pub const DELETED_FILE_NAME: &str = "//deleted.tsk";

    fn load_tasks_from_files(&self, files_directory_path: &String) -> Result<LoadResults, String> {
        let load_result = LoadResults::new();

        if self.load_options.load_cancelled {}

        if self.load_options.load_expired {}

        if self.load_options.load_completed {}

        if self.load_options.load_deleted {}

        Ok(LoadResults::new())
    }

    fn load_tasks_from_a_single_file(file_name: &str) -> Result<Vec<Task>, String> {
        fn error_out(param: Error) -> Result<String, String> {
            Err(param.to_string())
        }

        match fs::read_to_string(file_name).or_else(error_out) {
            Ok(task_data_string) => {
                let task_vector: Vec<Task> = task_data_string
                    .lines()
                    .map(|one_line| Task::from(one_line))
                    .collect();
                Ok(task_vector)
            }
            Err(error) => Err(error),
        }
    }
}
