#[cfg(test)]
mod tests {
    use super::*;

    fn manager() -> WorkingDirectoryManager {
        WorkingDirectoryManager {
            root: "/jobs".to_string(),
        }
    }

    #[test]
    fn get_directory_returns_job_directory() {
        let manager = manager();

        let result = manager.get_directory("job-123");

        assert_eq!(result, "/jobs/job-123");
    }

    #[test]
    fn get_directory_works_with_different_job_ids() {
        let manager = manager();

        assert_eq!(
            manager.get_directory("job-1"),
            "/jobs/job-1"
        );

        assert_eq!(
            manager.get_directory("job-999"),
            "/jobs/job-999"
        );
    }

    #[test]
    fn get_directory_works_with_empty_job_id() {
        let manager = manager();

        let result = manager.get_directory("");

        assert_eq!(result, "/jobs/");
    }

    #[test]
    fn get_directory_works_with_nested_root() {
        let manager = WorkingDirectoryManager {
            root: "/var/lib/gt-cloud/jobs".to_string(),
        };

        let result = manager.get_directory("job-123");

        assert_eq!(
            result,
            "/var/lib/gt-cloud/jobs/job-123"
        );
    }
}