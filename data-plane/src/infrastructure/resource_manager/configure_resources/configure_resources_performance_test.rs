use std::{collections::HashMap, time::Instant};

use crate::{
    domain::{
        app::App,
        job::{Job, JobStatus},
        resource::Resource,
    },
    infrastructure::resource_manager::{
        configure_resources::configure_resources::ConfigureResources,
        helpers::resource_path_manager::ResourcePathManager,
        states::resource_manager_state::ResourceManagerState,
    },
    ports::{logger::Logger, metrics::Metrics},
};

struct NoopLogger;

impl Logger for NoopLogger {
    fn info(&self, _message: &str, _fields: HashMap<String, String>) {}

    fn error(&self, _message: &str, _fields: HashMap<String, String>) {}
}

struct NoopMetrics;

impl Metrics for NoopMetrics {
    fn increment(&self, _name: &str, _value: f64) {}

    fn observe(&self, _name: &str, _value: f64) {}
}

fn make_configurer() -> ConfigureResources<NoopLogger, NoopMetrics> {
    ConfigureResources {
        resource_path_manager: ResourcePathManager {
            root: "/var/lib/gt-cloud/resources".to_string(),
        },
        logger: NoopLogger,
        metrics: NoopMetrics,
    }
}

fn make_job(id: usize) -> Job {
    Job {
        id: format!("job-{id}"),
        app: App {
            id: format!("app-{id}"),
            code_path: "/code".to_string(),
            command: "python app.py".to_string(),
        },
        name: format!("test-job-{id}"),
        status: JobStatus::Received,
        resources: Resource {
            cpu: 1,
            memory: 1024,
        },
        network: vec![],
    }
}

#[test]
fn configure_resources_performance() {
    for size in [1_000, 10_000, 100_000] {
        let configurer = make_configurer();
        let mut state = ResourceManagerState::default();
        let started_at = Instant::now();

        for id in 0..size {
            configurer
                .configure(&mut state, make_job(id))
                .expect("resource configuration should succeed");
        }

        let elapsed = started_at.elapsed();
        let average_time_ns = elapsed.as_nanos() / size as u128;
        let jobs_per_second = size as f64 / elapsed.as_secs_f64();

        assert_eq!(state.allocations.len(), size);

        println!(
            "Resource configure performance: {size} jobs | total: {} us | avg: {average_time_ns} ns/job | throughput: {jobs_per_second:.2} jobs/sec",
            elapsed.as_micros()
        );
    }
}
