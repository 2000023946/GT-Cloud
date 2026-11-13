/// Generates deterministic locations for job-specific resource controls.
pub struct ResourcePathManager {
    pub(crate) root: String,
}

impl ResourcePathManager {
    pub fn get_path(&self, job_id: &str) -> String {
        let root = self.root.trim_end_matches('/');

        if root.is_empty() {
            format!("/{job_id}")
        } else {
            format!("{root}/{job_id}")
        }
    }
}
