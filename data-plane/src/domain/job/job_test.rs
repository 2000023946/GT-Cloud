
#[cfg(test)]
mod tests {
    use crate::domain::job::{Job, JobStatus};

use super::*;

    #[test]
    fn test_new_job() {
        let job = Job::new(
            "job-001".to_string(),
            "test-job".to_string(),
            2.0,
            512.0,
            "/tmp/app".to_string(),
            "python main.py".to_string(),
        );

        // Job fields
        assert_eq!(job.id, "job-001");
        assert_eq!(job.name, "test-job");

        // App fields
        assert_eq!(job.app.id, "job-001");
        assert_eq!(job.app.code_path, "/tmp/app");
        assert_eq!(job.app.command, "python main.py");
        assert!(job.app.environment.is_empty());

        // Resource fields
        assert_eq!(job.resources.cpu, 2.0);
        assert_eq!(job.resources.memory, 512.0);

        // Default state
        assert!(matches!(job.status, JobStatus::Starting));

        // Default network configuration
        assert!(job.network.is_empty());
    }

    #[test]
    fn test_new_job_preserves_job_and_app_ids() {
        let job = Job::new(
            "unique-id-123".to_string(),
            "another-job".to_string(),
            1.0,
            256.0,
            "/app".to_string(),
            "./start.sh".to_string(),
        );

        assert_eq!(job.id, job.app.id);
    }

    #[test]
    fn test_new_job_with_different_resources() {
        let job = Job::new(
            "job-002".to_string(),
            "resource-test".to_string(),
            4.0,
            1024.0,
            "/app".to_string(),
            "node server.js".to_string(),
        );

        assert_eq!(job.resources.cpu, 4.0);
        assert_eq!(job.resources.memory, 1024.0);
    }
}
