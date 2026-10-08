pub struct WorkingDirectoryManager {
    root: String,
}

impl WorkingDirectoryManager {
    pub fn get_directory(&self, job_id: &str) -> String {
        format!("{}/{}", self.root, job_id)
    }
}