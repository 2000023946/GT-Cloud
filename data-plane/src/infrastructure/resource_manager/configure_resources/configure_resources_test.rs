use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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

type LogEntry = (String, HashMap<String, String>);

#[derive(Clone, Default)]
struct RecordingLogger {
    info_entries: Arc<Mutex<Vec<LogEntry>>>,
    error_entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl Logger for RecordingLogger {
    fn info(&self, message: &str, fields: HashMap<String, String>) {
        self.info_entries
            .lock()
            .unwrap()
            .push((message.to_string(), fields));
    }

    fn error(&self, message: &str, fields: HashMap<String, String>) {
        self.error_entries
            .lock()
            .unwrap()
            .push((message.to_string(), fields));
    }
}

#[derive(Clone, Default)]
struct RecordingMetrics {
    increments: Arc<Mutex<Vec<(String, f64)>>>,
    observations: Arc<Mutex<Vec<(String, f64)>>>,
}

impl Metrics for RecordingMetrics {
    fn increment(&self, name: &str, value: f64) {
        self.increments
            .lock()
            .unwrap()
            .push((name.to_string(), value));
    }

    fn observe(&self, name: &str, value: f64) {
        self.observations
            .lock()
            .unwrap()
            .push((name.to_string(), value));
    }
}

fn make_configurer(
    logger: RecordingLogger,
    metrics: RecordingMetrics,
) -> ConfigureResources<RecordingLogger, RecordingMetrics> {
    ConfigureResources {
        resource_path_manager: ResourcePathManager {
            root: "/var/lib/gt-cloud/resources".to_string(),
        },
        logger,
        metrics,
    }
}

fn make_job(id: &str, cpu: u32, memory: u64) -> Job {
    Job {
        id: id.to_string(),
        app: App {
            id: "app-123".to_string(),
            code_path: "/code".to_string(),
            command: "python app.py".to_string(),
            environment: vec![
                ("PORT".to_string(), "8080".to_string()),
                ("MODE".to_string(), "test".to_string()),
            ],
        },
        name: "test-job".to_string(),
        status: JobStatus::Received,
        resources: Resource { cpu, memory },
        network: vec![],
    }
}

#[test]
fn configure_stores_valid_allocation() {
    let configurer = make_configurer(Default::default(), Default::default());
    let mut state = ResourceManagerState::default();

    let result = configurer.configure(&mut state, make_job("job-123", 2, 4096));

    assert!(result.is_ok());
    assert_eq!(state.allocations.get("job-123").unwrap().cpu, 2);
    assert_eq!(state.allocations.get("job-123").unwrap().memory, 4096);
    assert_eq!(
        state.allocations.get("job-123").unwrap().resource_path,
        "/var/lib/gt-cloud/resources/job-123"
    );
}

#[test]
fn configure_stores_multiple_jobs_separately() {
    let configurer = make_configurer(Default::default(), Default::default());
    let mut state = ResourceManagerState::default();

    configurer
        .configure(&mut state, make_job("job-1", 1, 1024))
        .unwrap();
    configurer
        .configure(&mut state, make_job("job-2", 2, 2048))
        .unwrap();

    assert_eq!(state.allocations.len(), 2);
    assert_eq!(state.allocations.get("job-1").unwrap().cpu, 1);
    assert_eq!(state.allocations.get("job-2").unwrap().cpu, 2);
}

#[test]
fn configure_replaces_existing_valid_allocation() {
    let configurer = make_configurer(Default::default(), Default::default());
    let mut state = ResourceManagerState::default();

    configurer
        .configure(&mut state, make_job("job-123", 1, 1024))
        .unwrap();
    configurer
        .configure(&mut state, make_job("job-123", 4, 8192))
        .unwrap();

    let allocation = state.allocations.get("job-123").unwrap();
    assert_eq!(state.allocations.len(), 1);
    assert_eq!(allocation.cpu, 4);
    assert_eq!(allocation.memory, 8192);
}

#[test]
fn configure_rejects_empty_job_id() {
    assert_configuration_fails(make_job("", 1, 1024), "Job ID cannot be empty");
}

#[test]
fn configure_rejects_whitespace_job_id() {
    assert_configuration_fails(make_job("   ", 1, 1024), "Job ID cannot be empty");
}

#[test]
fn configure_rejects_unsafe_job_id() {
    assert_configuration_fails(
        make_job("../job/123", 1, 1024),
        "Job ID may contain only letters, numbers, hyphens, and underscores",
    );
}

#[test]
fn configure_rejects_zero_cpu() {
    assert_configuration_fails(
        make_job("job-123", 0, 1024),
        "CPU allocation must be greater than zero",
    );
}

#[test]
fn configure_rejects_zero_memory() {
    assert_configuration_fails(
        make_job("job-123", 1, 0),
        "Memory allocation must be greater than zero",
    );
}

#[test]
fn failed_reconfiguration_preserves_previous_allocation() {
    let configurer = make_configurer(Default::default(), Default::default());
    let mut state = ResourceManagerState::default();

    configurer
        .configure(&mut state, make_job("job-123", 1, 1024))
        .unwrap();
    let result = configurer.configure(&mut state, make_job("job-123", 0, 2048));

    assert!(result.is_err());
    let allocation = state.allocations.get("job-123").unwrap();
    assert_eq!(allocation.cpu, 1);
    assert_eq!(allocation.memory, 1024);
}

#[test]
fn configure_records_success_observability() {
    let logger = RecordingLogger::default();
    let metrics = RecordingMetrics::default();
    let configurer = make_configurer(logger.clone(), metrics.clone());
    let mut state = ResourceManagerState::default();

    configurer
        .configure(&mut state, make_job("job-123", 1, 1024))
        .unwrap();

    let info_entries = logger.info_entries.lock().unwrap();
    assert_eq!(info_entries.len(), 1);
    assert_eq!(info_entries[0].0, "Resources configured for job");
    assert_eq!(info_entries[0].1.get("job_id").unwrap(), "job-123");

    let increments = metrics.increments.lock().unwrap();
    assert!(increments.contains(&("resource.configure.success".to_string(), 1.0)));
    assert!(
        !increments
            .iter()
            .any(|(name, _)| name == "resource.configure.failure")
    );

    let observations = metrics.observations.lock().unwrap();
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].0, "resource.configure.duration_ms");
    assert!(observations[0].1 >= 0.0);
}

#[test]
fn configure_records_failure_observability() {
    let logger = RecordingLogger::default();
    let metrics = RecordingMetrics::default();
    let configurer = make_configurer(logger.clone(), metrics.clone());
    let mut state = ResourceManagerState::default();

    let result = configurer.configure(&mut state, make_job("job-123", 0, 1024));

    assert!(result.is_err());
    let error_entries = logger.error_entries.lock().unwrap();
    assert_eq!(error_entries.len(), 1);
    assert_eq!(error_entries[0].0, "Failed to configure resources");
    assert_eq!(error_entries[0].1.get("job_id").unwrap(), "job-123");
    assert_eq!(
        error_entries[0].1.get("reason").unwrap(),
        "CPU allocation must be greater than zero"
    );

    let increments = metrics.increments.lock().unwrap();
    assert!(increments.contains(&("resource.configure.failure".to_string(), 1.0)));
    assert!(
        !increments
            .iter()
            .any(|(name, _)| name == "resource.configure.success")
    );

    let observations = metrics.observations.lock().unwrap();
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].0, "resource.configure.duration_ms");
}

fn assert_configuration_fails(job: Job, expected_error: &str) {
    let configurer = make_configurer(Default::default(), Default::default());
    let mut state = ResourceManagerState::default();

    let result = configurer.configure(&mut state, job);

    assert_eq!(result.unwrap_err(), expected_error);
    assert!(state.allocations.is_empty());
}
