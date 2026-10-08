#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::app::App;
    use crate::domain::job::{Job, JobStatus};
    use crate::domain::network::NetworkRule;
    use crate::domain::resource::Resource;
    use crate::infrastructure::runtime::working_directory::WorkingDirectoryManager;

    fn make_manager() -> WorkingDirectoryManager {
        WorkingDirectoryManager {
            root: "/jobs".to_string(),
        }
    }

    fn make_runtime() -> ConfigureRuntime {
        ConfigureRuntime {
            working_directory_manager: make_manager(),
        }
    }

    fn make_job(
        id: &str,
        command: &str,
        environment: Vec<(String, String)>,
    ) -> Job {
        Job {
            id: id.to_string(),
            app: App {
                id: format!("app-{id}"),
                code_path: "/some/code/path".to_string(),
                command: command.to_string(),
                environment,
            },
            name: format!("job-{id}"),
            status: JobStatus::Pending,
            resources: Resource::default(),
            network: Vec::<NetworkRule>::new(),
        }
    }

    fn make_state() -> RuntimeState {
        RuntimeState::new()
    }

    #[test]
    fn configure_stores_process_state() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python main.py",
            vec![],
        );

        let result = runtime.configure(&mut state, job);

        assert!(result.is_ok());

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.program, "python");
        assert_eq!(process.args, vec!["main.py"]);
    }

    #[test]
    fn configure_uses_working_directory_manager() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job = make_job(
            "job-123",
            "python main.py",
            vec![],
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-123").unwrap();

        assert_eq!(
            process.working_directory,
            Some("/jobs/job-123".to_string())
        );
    }

    #[test]
    fn configure_stores_environment() {
        let runtime = make_runtime();
        let mut state = make_state();

        let environment = vec![
            ("PORT".to_string(), "8080".to_string()),
            ("MODE".to_string(), "production".to_string()),
        ];

        let job = make_job(
            "job-1",
            "python main.py",
            environment.clone(),
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.environment, environment);
    }

    #[test]
    fn configure_stores_command_arguments() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python main.py --port 8080",
            vec![],
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.program, "python");

        assert_eq!(
            process.args,
            vec![
                "main.py".to_string(),
                "--port".to_string(),
                "8080".to_string(),
            ]
        );
    }

    #[test]
    fn configure_accepts_command_without_arguments() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python",
            vec![],
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.program, "python");
        assert!(process.args.is_empty());
    }

    #[test]
    fn configure_rejects_empty_command() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "",
            vec![],
        );

        let result = runtime.configure(&mut state, job);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Command cannot be empty"
        );

        assert!(!state.processes.contains_key("job-1"));
    }

    #[test]
    fn configure_rejects_whitespace_only_command() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "     ",
            vec![],
        );

        let result = runtime.configure(&mut state, job);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Command cannot be empty"
        );

        assert!(!state.processes.contains_key("job-1"));
    }

    #[test]
    fn configure_replaces_existing_job() {
        let runtime = make_runtime();
        let mut state = make_state();

        let first_job = make_job(
            "job-1",
            "python first.py",
            vec![],
        );

        runtime.configure(&mut state, first_job).unwrap();

        let second_job = make_job(
            "job-1",
            "python second.py",
            vec![],
        );

        runtime.configure(&mut state, second_job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.args, vec!["second.py"]);
        assert_eq!(state.processes.len(), 1);
    }

    #[test]
    fn configure_supports_multiple_jobs() {
        let runtime = make_runtime();
        let mut state = make_state();

        let job1 = make_job(
            "job-1",
            "python app1.py",
            vec![],
        );

        let job2 = make_job(
            "job-2",
            "python app2.py",
            vec![],
        );

        runtime.configure(&mut state, job1).unwrap();
        runtime.configure(&mut state, job2).unwrap();

        assert_eq!(state.processes.len(), 2);

        assert_eq!(
            state.processes.get("job-1").unwrap().args,
            vec!["app1.py"]
        );

        assert_eq!(
            state.processes.get("job-2").unwrap().args,
            vec!["app2.py"]
        );
    }
}