use crate::model::{LoadHandler, LoadOptions, LoadResult};

pub struct FileLoadHandler {}
impl LoadHandler for FileLoadHandler {
    fn load_tasks(options: LoadOptions) -> Result<Self, String> {
        Ok(FileLoadHandler {})
    }
}
