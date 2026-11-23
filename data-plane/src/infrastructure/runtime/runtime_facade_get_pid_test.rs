use std::collections::HashMap;

use crate::{domain::{app::App, job::{Job, JobStatus}, resource::Resource}, infrastructure::runtime::{runtime_facade::RuntimeFacade, states::process_state::ProcessState}, ports::{logger::Logger, metrics::Metrics}};

    #[derive(Clone)]
    struct TestLogger;

    impl Logger for TestLogger {
        fn info(&self, message: &str, fields: HashMap<String, String>) {
            println!("[INFO] {} {:?}", message, fields);
        }

        fn error(&self, message: &str, fields: HashMap<String, String>) {
            eprintln!("[ERROR] {} {:?}", message, fields);
        }
    }

    #[derive(Clone)]
    struct TestMetrics;

    impl Metrics for TestMetrics {
        fn increment(&self, name: &str, value: f64) {
            println!("[METRIC] {} += {}", name, value);
        }

        fn observe(&self, name: &str, value: f64) {
            println!("[METRIC] {} observed {}", name, value);
        }
    }

        fn create_test_job() -> Job {
        Job {
            id: "integration-test-001".to_string(),
            name: "Integration Test".to_string(),

            app: App {
                id: "test-app".to_string(),
                code_path: String::new(),
                command: "sleep 30".to_string(),
                environment: vec![],
            },

            status: JobStatus::Received,

            resources: Resource {
                cpu: 1.0,
                memory: (128 * 1024 * 1024) as f32,
            },

            network: vec![],
        }
    }

#[test]
fn test_get_pid_process_not_found() {
    let facade = RuntimeFacade::new(
        TestLogger{},
        TestMetrics{},
    );

    let job = create_test_job();

    let result = facade.get_pid(&job);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Process does not exist!");
}

#[test]
fn test_get_pid_pid_not_found() {
    let facade = RuntimeFacade::new(
        TestLogger{},
        TestMetrics{},
    );

    let job = create_test_job();

    let process = ProcessState {
        program: "test".to_string(),
        args: vec![],
        working_directory: None,
        environment: vec![],
        pid: None,
    };

    facade.runtime_state().add_process(job.id.clone(), process);

    let result = facade.get_pid(&job);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Pid does not exist!");
}

#[test]
fn test_get_pid_success() {
    let facade = RuntimeFacade::new(
        TestLogger{},
        TestMetrics{},
    );

    let job = create_test_job();

    let process = ProcessState {
        program: "test".to_string(),
        args: vec![],
        working_directory: None,
        environment: vec![],
        pid: Some(12345),
    };

    facade.runtime_state().add_process(job.id.clone(), process);

    let result = facade.get_pid(&job);

    assert_eq!(result.unwrap(), 12345);
}