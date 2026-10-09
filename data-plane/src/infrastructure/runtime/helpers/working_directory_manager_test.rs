
use crate::infrastructure::runtime::helpers::working_directory_manager::WorkingDirectoryManager;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn manager() -> WorkingDirectoryManager {
        WorkingDirectoryManager {
            root: "/jobs".to_string(),
        }
    }

    #[test]
    fn get_directory_returns_job_directory() {
        let manager = manager();

        assert_eq!(
            manager.get_directory("job-123"),
            "/jobs/job-123"
        );
    }

    #[test]
    fn get_directory_works_with_different_job_ids() {
        let manager = manager();

        assert_eq!(manager.get_directory("job-1"), "/jobs/job-1");
        assert_eq!(manager.get_directory("job-999"), "/jobs/job-999");
    }

    #[test]
    fn get_directory_works_with_empty_job_id() {
        let manager = manager();

        assert_eq!(manager.get_directory(""), "/jobs/");
    }

    #[test]
    fn get_directory_works_with_nested_root() {
        let manager = WorkingDirectoryManager {
            root: "/var/lib/gt-cloud/jobs".to_string(),
        };

        assert_eq!(
            manager.get_directory("job-123"),
            "/var/lib/gt-cloud/jobs/job-123"
        );
    }

    #[test]
    fn create_directory_creates_job_directory() {
        let root = std::env::temp_dir().join(format!(
            "gt-cloud-test-{}-job-123",
            std::process::id()
        ));

        // Ensure this test starts with a clean directory.
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }

        let manager = WorkingDirectoryManager {
            root: root.to_string_lossy().into_owned(),
        };

        let directory = manager
            .create_directory("job-123")
            .expect("Failed to create job working directory");

        assert_eq!(
            directory,
            root.join("job-123").to_string_lossy()
        );

        assert!(
            Path::new(&directory).is_dir(),
            "Expected working directory to exist: {}",
            directory
        );

        fs::remove_dir_all(&root).unwrap();
    }
}