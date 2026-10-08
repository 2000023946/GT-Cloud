#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::app::App;
    use crate::domain::job::{Job, JobStatus};
    use crate::domain::network::NetworkRule;
    use crate::domain::resource::Resource;
    use crate::infrastructure::runtime::state::RuntimeState;

    fn make_job(
        id: &str,
        command: &str,
        code_path: &str,
        environment: Vec<(String, String)>,
    ) -> Job {
        Job {
            id: id.to_string(),
            app: App {
                id: format!("app-{id}"),
                code_path: code_path.to_string(),
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
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python main.py",
            "/jobs/job-1",
            vec![],
        );

        let result = runtime.configure(&mut state, job);

        assert!(result.is_ok());

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.program, "python");
        assert_eq!(process.args, vec!["main.py"]);
        assert_eq!(
            process.working_directory,
            Some("/jobs/job-1".to_string())
        );
    }

    #[test]
    fn configure_stores_command_arguments() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python main.py --port 8080",
            "/jobs/job-1",
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
                "8080".to_string()
            ]
        );
    }

    #[test]
    fn configure_stores_environment() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let environment = vec![
            ("PORT".to_string(), "8080".to_string()),
            ("MODE".to_string(), "production".to_string()),
        ];

        let job = make_job(
            "job-1",
            "server",
            "/jobs/job-1",
            environment.clone(),
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.environment, environment);
    }

    #[test]
    fn configure_stores_working_directory() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python main.py",
            "/jobs/job-1",
            vec![],
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(
            process.working_directory,
            Some("/jobs/job-1".to_string())
        );
    }

    #[test]
    fn configure_rejects_empty_command() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "",
            "/jobs/job-1",
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
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "     ",
            "/jobs/job-1",
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
    fn configure_accepts_command_without_arguments() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "job-1",
            "python",
            "/jobs/job-1",
            vec![],
        );

        runtime.configure(&mut state, job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.program, "python");
        assert!(process.args.is_empty());
    }

    #[test]
    fn configure_uses_job_id_as_state_key() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job = make_job(
            "my-job",
            "python main.py",
            "/jobs/my-job",
            vec![],
        );

        runtime.configure(&mut state, job).unwrap();

        assert!(state.processes.contains_key("my-job"));
    }

    #[test]
    fn configure_replaces_existing_job_with_same_id() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let first_job = make_job(
            "job-1",
            "python first.py",
            "/jobs/first",
            vec![],
        );

        runtime.configure(&mut state, first_job).unwrap();

        let second_job = make_job(
            "job-1",
            "python second.py",
            "/jobs/second",
            vec![],
        );

        runtime.configure(&mut state, second_job).unwrap();

        let process = state.processes.get("job-1").unwrap();

        assert_eq!(process.program, "python");
        assert_eq!(process.args, vec!["second.py"]);
        assert_eq!(
            process.working_directory,
            Some("/jobs/second".to_string())
        );

        assert_eq!(state.processes.len(), 1);
    }

    #[test]
    fn configure_supports_multiple_jobs() {
        let runtime = ConfigureRuntime;
        let mut state = make_state();

        let job1 = make_job(
            "job-1",
            "python app1.py",
            "/jobs/job-1",
            vec![],
        );

        let job2 = make_job(
            "job-2",
            "python app2.py",
            "/jobs/job-2",
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