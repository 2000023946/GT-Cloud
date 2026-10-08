use crate::domain::app::App;
use crate::domain::job::{Job, JobStatus};
use crate::domain::network::NetworkRule;
use crate::domain::resource::Resource;
use crate::infrastructure::runtime::configure_runtime::configure_runtime::ConfigureRuntime;
use crate::infrastructure::runtime::helpers::working_directory_manager::WorkingDirectoryManager;
use crate::infrastructure::runtime::states::runtime_state::RuntimeState;
use crate::observability::observability::Observability;
use crate::ports::logger::Logger;
use crate::ports::metrics::Metrics;
use std::collections::HashMap;

struct TestLogger;

impl Logger for TestLogger {
    fn info(&self, _message: &str, _fields: HashMap<String, String>) {}

    fn error(&self, _message: &str, _fields: HashMap<String, String>) {}
}

struct TestMetrics;

impl Metrics for TestMetrics {
    fn increment(&self, _name: &str, _value: f64) {}

    fn observe(&self, _name: &str, _value: f64) {}
}

fn make_manager() -> WorkingDirectoryManager {
    WorkingDirectoryManager {
        root: "/jobs".to_string(),
    }
}

fn make_runtime() -> ConfigureRuntime<TestLogger, TestMetrics> {
    ConfigureRuntime {
        working_directory_manager: make_manager(),
        observability: Observability {
            logger: TestLogger,
            metrics: TestMetrics,
        },
    }
}

fn make_job(command: &str) -> Job {
    Job {
        id: "job-123".to_string(),
        app: App {
            id: "app-123".to_string(),
            code_path: "/code".to_string(),
            command: command.to_string(),
            environment: vec![],
        },
        name: "test-job".to_string(),
        status: JobStatus::Received,
        resources: Resource {
            cpu: 0,
            memory: 0,
        },
        network: Vec::<NetworkRule>::new(),
    }
}

fn make_state() -> RuntimeState {
    RuntimeState {
        processes: std::collections::HashMap::new(),
    }
}

#[test]
fn configure_valid_command() {
    let runtime = make_runtime();
    let mut state = make_state();
    let job = make_job("python app.py");

    let result = runtime.configure(&mut state, job);

    assert!(result.is_ok());

    let process = state.processes.get("job-123").unwrap();

    assert_eq!(process.program, "python");
    assert_eq!(process.args, vec!["app.py"]);
    assert_eq!(
        process.working_directory,
        Some("/jobs/job-123".to_string())
    );
}

#[test]
fn configure_command_with_multiple_arguments() {
    let runtime = make_runtime();
    let mut state = make_state();
    let job = make_job("python app.py --port 8080");

    let result = runtime.configure(&mut state, job);

    assert!(result.is_ok());

    let process = state.processes.get("job-123").unwrap();

    assert_eq!(process.program, "python");
    assert_eq!(
        process.args,
        vec!["app.py", "--port", "8080"]
    );
}

#[test]
fn configure_command_without_arguments() {
    let runtime = make_runtime();
    let mut state = make_state();
    let job = make_job("python");

    let result = runtime.configure(&mut state, job);

    assert!(result.is_ok());

    let process = state.processes.get("job-123").unwrap();

    assert_eq!(process.program, "python");
    assert!(process.args.is_empty());
}

#[test]
fn configure_sets_environment() {
    let runtime = make_runtime();
    let mut state = make_state();

    let mut job = make_job("python app.py");

    job.app.environment = vec![
        ("PORT".to_string(), "8080".to_string()),
        ("MODE".to_string(), "test".to_string()),
    ];

    let result = runtime.configure(&mut state, job);

    assert!(result.is_ok());

    let process = state.processes.get("job-123").unwrap();

    assert_eq!(
        process.environment,
        vec![
            ("PORT".to_string(), "8080".to_string()),
            ("MODE".to_string(), "test".to_string()),
        ]
    );
}

#[test]
fn configure_empty_command_fails() {
    let runtime = make_runtime();
    let mut state = make_state();
    let job = make_job("");

    let result = runtime.configure(&mut state, job);

    assert!(result.is_err());
}

#[test]
fn configure_whitespace_command_fails() {
    let runtime = make_runtime();
    let mut state = make_state();
    let job = make_job("   ");

    let result = runtime.configure(&mut state, job);

    assert!(result.is_err());
}

#[test]
fn configure_replaces_existing_job() {
    let runtime = make_runtime();
    let mut state = make_state();

    let job1 = make_job("python app.py");
    runtime.configure(&mut state, job1).unwrap();

    let job2 = make_job("node server.js");
    runtime.configure(&mut state, job2).unwrap();

    let process = state.processes.get("job-123").unwrap();

    assert_eq!(process.program, "node");
    assert_eq!(process.args, vec!["server.js"]);

    assert_eq!(state.processes.len(), 1);
}

#[test]
fn configure_multiple_jobs() {
    let runtime = make_runtime();
    let mut state = make_state();

    let job1 = make_job("python app.py");

    let mut job2 = make_job("node server.js");
    job2.id = "job-456".to_string();

    runtime.configure(&mut state, job1).unwrap();
    runtime.configure(&mut state, job2).unwrap();

    assert_eq!(state.processes.len(), 2);

    assert_eq!(
        state.processes.get("job-123").unwrap().program,
        "python"
    );

    assert_eq!(
        state.processes.get("job-456").unwrap().program,
        "node"
    );
}