
#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::super::child_process::{
        append_exec_error,
        create_log_file,
        ChildProcess,
    };

    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_dir() -> PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir()
            .join(format!("child-process-test-{id}"));

        fs::create_dir_all(&path).unwrap();
        path
    }

    fn make_context(
        program: &str,
        args: Vec<String>,
        working_directory: Option<String>,
        environment: Vec<(String, String)>,
    ) -> *mut libc::c_void {
        Box::into_raw(Box::new(ChildProcess {
            program: program.to_string(),
            args,
            working_directory,
            environment,
        })) as *mut libc::c_void
    }

    #[test]
    fn null_context_returns_127() {
        assert_eq!(ChildProcess::run(std::ptr::null_mut()), 127);
    }

    #[test]
    fn creates_log_file() {
        let dir = temp_dir();
        let path = dir.join("stdout.txt");

        let result = create_log_file(&path);

        assert!(result.is_ok());
        assert!(path.exists());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn creates_fresh_log_file() {
        let dir = temp_dir();
        let path = dir.join("stdout.txt");

        fs::write(&path, "old contents").unwrap();

        let file = create_log_file(&path).unwrap();
        drop(file);

        assert_eq!(fs::read_to_string(&path).unwrap(), "");

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn log_creation_failure_returns_error() {
        let dir = temp_dir();
        let path = dir.join("missing").join("stdout.txt");

        assert!(create_log_file(&path).is_err());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn appends_exec_error_to_stderr_log() {
        let dir = temp_dir();
        let path = dir.join("stderr.txt");

        fs::write(&path, "previous output\n").unwrap();

        let error = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "executable not found",
        );

        let args = vec!["--example".to_string()];

        append_exec_error(
            &path,
            "/missing/program",
            &args,
            &error,
        );

        let contents = fs::read_to_string(&path).unwrap();

        assert!(contents.contains("previous output"));
        assert!(contents.contains("Failed to execute"));
        assert!(contents.contains("/missing/program"));
        assert!(contents.contains("--example"));
        assert!(contents.contains("executable not found"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn handles_stderr_append_failure() {
        let dir = temp_dir();
        let path = dir.join("missing").join("stderr.txt");

        let error = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "executable not found",
        );

        append_exec_error(
            &path,
            "/missing/program",
            &[],
            &error,
        );

        assert!(!path.exists());

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn missing_executable_returns_127_and_logs_error() {
        let dir = temp_dir();
        let stderr_path = dir.join("stderr.txt");

        let context = make_context(
            "/definitely/not/a/real/executable",
            vec![],
            Some(dir.to_string_lossy().into_owned()),
            vec![],
        );

        let result = ChildProcess::run(context);

        assert_eq!(result, 127);

        let stderr = fs::read_to_string(stderr_path).unwrap();
        assert!(stderr.contains("Failed to execute"));
        assert!(stderr.contains("/definitely/not/a/real/executable"));

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn run_fails_when_working_directory_does_not_exist() {
        let dir = temp_dir();
        let missing_dir = dir.join("does-not-exist");

        let context = make_context(
            "/bin/echo",
            vec!["hello".to_string()],
            Some(missing_dir.to_string_lossy().into_owned()),
            vec![],
        );

        // stdout log creation fails before the command is executed.
        assert_eq!(ChildProcess::run(context), 127);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn run_fails_when_stderr_path_is_a_directory() {
        let dir = temp_dir();
        let stderr_path = dir.join("stderr.txt");

        fs::create_dir(&stderr_path).unwrap();

        let context = make_context(
            "/bin/echo",
            vec!["hello".to_string()],
            Some(dir.to_string_lossy().into_owned()),
            vec![],
        );

        assert_eq!(ChildProcess::run(context), 127);

        fs::remove_dir_all(dir).unwrap();
    }
}