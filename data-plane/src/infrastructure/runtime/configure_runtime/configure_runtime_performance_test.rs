use std::collections::HashMap;
use std::time::Instant;

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

fn make_job(id: usize) -> Job {
    Job {
        id: format!("job-{}", id),
        app: App {
            id: format!("app-{}", id),
            code_path: "/code".to_string(),
            command: "python app.py --port 8080".to_string(),
            environment: vec![
                ("MODE".to_string(), "test".to_string()),
            ],
        },
        name: format!("test-job-{}", id),
        status: JobStatus::Received,
        resources: Resource {
            cpu: 1,
            memory: 1024,
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
fn configure_performance_test() {
    let test_sizes = [1_000, 10_000, 100_000];

    for size in test_sizes {
        let runtime = make_runtime();
        let mut state = make_state();

        let start = Instant::now();

        for i in 0..size {
            let job = make_job(i);

            runtime
                .configure(&mut state, job)
                .expect("configure should succeed");
        }

        let elapsed = start.elapsed();

        let total_time_us = elapsed.as_micros();
        let average_time_ns = elapsed.as_nanos() / size as u128;
        let jobs_per_second =
            size as f64 / elapsed.as_secs_f64();

        assert_eq!(
            state.processes.len(),
            size,
            "Expected {} configured jobs, got {}",
            size,
            state.processes.len()
        );

        println!(
            "Configure performance: {} jobs | total: {} µs | avg: {} ns/job | throughput: {:.2} jobs/sec",
            size,
            total_time_us,
            average_time_ns,
            jobs_per_second
        );
    }
}