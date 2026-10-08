pub struct WorkingDirectoryManager {
    pub(crate) root: String,
}

impl WorkingDirectoryManager {
    pub fn get_directory(&self, job_id: &str) -> String {
        format!("{}/{}", self.root, job_id)
    }
}