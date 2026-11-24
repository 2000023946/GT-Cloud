
#[cfg(test)]
mod tests {
    use crate::{applications::get_job_logs::{get_job_logs_request::GetJobLogsRequest, get_job_logs_use_case::GetJobLogsUseCase}, infrastructure::runtime::helpers::working_directory_manager::WorkingDirectoryManager, ports::i_logs_reader::ILogsReader};

use super::*;
    use std::sync::Mutex;

    struct MockLogsReader {
        stdout_result: Result<String, String>,
        stderr_result: Result<String, String>,
        calls: Mutex<Vec<(String, String)>>,
    }

    impl ILogsReader for MockLogsReader {
        fn read_stdout(&self, path: String) -> Result<String, String> {
            self.calls
                .lock()
                .unwrap()
                .push(("stdout".to_string(), path));

            self.stdout_result.clone()
        }

        fn read_stderr(&self, path: String) -> Result<String, String> {
            self.calls
                .lock()
                .unwrap()
                .push(("stderr".to_string(), path));

            self.stderr_result.clone()
        }
    }

    fn create_mock(
        stdout: Result<String, String>,
        stderr: Result<String, String>,
    ) -> MockLogsReader {
        MockLogsReader {
            stdout_result: stdout,
            stderr_result: stderr,
            calls: Mutex::new(Vec::new()),
        }
    }

    #[test]
    fn test_execute_success() {
        let logs_reader = create_mock(
            Ok("Application started".to_string()),
            Ok("No errors".to_string()),
        );

        let use_case = GetJobLogsUseCase::new(
            WorkingDirectoryManager::new(),
            logs_reader,
        );

        let request = GetJobLogsRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(!response.is_error);
        assert_eq!(response.message, "");
        assert_eq!(response.stdout, "Application started");
        assert_eq!(response.stderr, "No errors");

        let calls = use_case.logs_reader.calls.lock().unwrap();

        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, "stderr");
        assert_eq!(calls[1].0, "stdout");

        assert_eq!(
            calls[0].1,
            "/tmp/working_directory/job-123"
        );
        assert_eq!(calls[1].1, calls[0].1);
    }

    #[test]
    fn test_execute_empty_logs() {
        let logs_reader = create_mock(
            Ok(String::new()),
            Ok(String::new()),
        );

        let use_case = GetJobLogsUseCase::new(
            WorkingDirectoryManager::new(),
            logs_reader,
        );

        let request = GetJobLogsRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(!response.is_error);
        assert_eq!(response.message, "");
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");
    }

    #[test]
    fn test_execute_stdout_error() {
        let logs_reader = create_mock(
            Err("Failed to read stdout".to_string()),
            Ok("No errors".to_string()),
        );

        let use_case = GetJobLogsUseCase::new(
            WorkingDirectoryManager::new(),
            logs_reader,
        );

        let request = GetJobLogsRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert_eq!(response.message, "Failed to read stdout");
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");

        let calls = use_case.logs_reader.calls.lock().unwrap();

        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, "stderr");
        assert_eq!(calls[1].0, "stdout");
    }

    #[test]
    fn test_execute_stderr_error() {
        let logs_reader = create_mock(
            Ok("Application started".to_string()),
            Err("Failed to read stderr".to_string()),
        );

        let use_case = GetJobLogsUseCase::new(
            WorkingDirectoryManager::new(),
            logs_reader,
        );

        let request = GetJobLogsRequest {
            job_id: "job-123".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert_eq!(response.message, "Failed to read stderr");
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");

        let calls = use_case.logs_reader.calls.lock().unwrap();

        // stdout should never be called when stderr fails.
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "stderr");
    }

    #[test]
    fn test_execute_directory_creation_error() {
        let logs_reader = create_mock(
            Ok("Application started".to_string()),
            Ok("No errors".to_string()),
        );

        let use_case = GetJobLogsUseCase::new(
            WorkingDirectoryManager::new(),
            logs_reader,
        );

        let request = GetJobLogsRequest {
            job_id: "\0".to_string(),
        };

        let response = use_case.execute(request);

        assert!(response.is_error);
        assert!(!response.message.is_empty());
        assert_eq!(response.stdout, "");
        assert_eq!(response.stderr, "");

        // Neither log reader should be called.
        let calls = use_case.logs_reader.calls.lock().unwrap();
        assert!(calls.is_empty());
    }
}
