
#[cfg(test)]
mod tests {
    use crate::infrastructure::logs_reader::logs_reader::LogsReader;
use crate::ports::i_file_reader::IFileReader;
use crate::ports::i_logs_reader::ILogsReader;

use super::*;
    use std::cell::RefCell;
    use std::io;

    struct MockFileReader {
        paths: RefCell<Vec<String>>,
        should_fail: bool,
    }

    impl MockFileReader {
        fn new(should_fail: bool) -> Self {
            Self {
                paths: RefCell::new(Vec::new()),
                should_fail,
            }
        }
    }

    impl IFileReader for MockFileReader {
        fn read(&self, path: String) -> Result<String, String> {
            self.paths.borrow_mut().push(path);

            if self.should_fail {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "file not found",
                ).to_string());
            }

            Ok("test log content".to_string())
        }
    }

    #[test]
    fn test_read_stdout_success() {
        let file_reader = MockFileReader::new(false);
        let logs_reader = LogsReader { file_reader };

        let result = logs_reader.read_stdout("/tmp/job-123".to_string());

        assert_eq!(result.unwrap(), "test log content");
        assert_eq!(
            logs_reader.file_reader.paths.borrow().as_slice(),
            ["/tmp/job-123/stdout.txt"]
        );
    }

    #[test]
    fn test_read_stderr_success() {
        let file_reader = MockFileReader::new(false);
        let logs_reader = LogsReader { file_reader };

        let result = logs_reader.read_stderr("/tmp/job-123".to_string());

        assert_eq!(result.unwrap(), "test log content");
        assert_eq!(
            logs_reader.file_reader.paths.borrow().as_slice(),
            ["/tmp/job-123/stderr.txt"]
        );
    }

    #[test]
    fn test_read_stdout_failure() {
        let file_reader = MockFileReader::new(true);
        let logs_reader = LogsReader { file_reader };

        let result = logs_reader.read_stdout("/tmp/job-123".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "file not found");
        assert_eq!(
            logs_reader.file_reader.paths.borrow().as_slice(),
            ["/tmp/job-123/stdout.txt"]
        );
    }

    #[test]
    fn test_read_stderr_failure() {
        let file_reader = MockFileReader::new(true);
        let logs_reader = LogsReader { file_reader };

        let result = logs_reader.read_stderr("/tmp/job-123".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "file not found");
        assert_eq!(
            logs_reader.file_reader.paths.borrow().as_slice(),
            ["/tmp/job-123/stderr.txt"]
        );
    }
}
