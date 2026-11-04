
use std::fs;
use std::io;

pub struct WorkingDirectoryManager {
    pub(crate) root: String,
}

impl WorkingDirectoryManager {
    pub fn get_directory(&self, job_id: &str) -> String {
        format!("{}/{}", self.root, job_id)
    }

    pub fn create_directory(&self, job_id: &str) -> io::Result<String> {
        let directory = self.get_directory(job_id);
        fs::create_dir_all(&directory)?;
        Ok(directory)
    }
}