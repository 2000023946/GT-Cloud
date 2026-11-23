
use super::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::applications::start_job::start_job_request::StartJobRequest;
use crate::applications::start_job::start_job_use_case::StartJobUseCase;
use crate::domain::job::Job;
use crate::domain::resource_states::resource_states::ResourceStates;
use crate::domain::runtime_states::runtime_states::RuntimeStates;
use crate::ports::i_code_repository::ICodeRepository;
use crate::ports::resource_manager::ResourceManager;
use crate::ports::runtime::Runtime;

// Shared mock state
#[derive(Clone)]
struct MockControl {
    events: Rc<RefCell<Vec<&'static str>>>,
    fail_at: Option<&'static str>,
}

impl MockControl {
    fn record(&self, event: &'static str) -> Result<(), String> {
        self.events.borrow_mut().push(event);

        if self.fail_at == Some(event) {
            return Err(format!("{} failed", event));
        }

        Ok(())
    }
}

// Mock code repository
struct MockCodeRepository {
    control: MockControl,
}

impl ICodeRepository for MockCodeRepository {
    fn create(&self, _code: Vec<u8>, _path: String) -> Result<(), String> {
        self.control.record("repository.create")
    }
}

// Mock resource manager
struct MockResourceManager {
    control: MockControl,
}

impl ResourceManager for MockResourceManager {
    fn configure(&self, _job: &Job) -> Result<(), String> {
        self.control.record("resource.configure")
    }

    fn start(&self, _job: &Job, _pid: u32) -> Result<(), String> {
        self.control.record("resource.start")
    }

    fn monitor(&self, _job: &Job) -> Result<ResourceStates, String> {
        Err("Not implemented".to_string())
    }

    fn cleanup(&self, _job: &Job) -> Result<(), String> {
        Err("Not implemented".to_string())
    }
}

// Mock runtime manager
struct MockRuntimeManager {
    control: MockControl,
}

impl Runtime for MockRuntimeManager {
    fn configure(&self, _job: &Job) -> Result<(), String> {
        self.control.record("runtime.configure")
    }

    fn start(&self, _job: &Job) -> Result<(), String> {
        self.control.record("runtime.start")
    }

    fn get_pid(&self, _job: &Job) -> Result<u32, String> {
        self.control.record("runtime.get_pid")?;
        Ok(12345)
    }

    fn monitor(&self, _job: &Job) -> Result<RuntimeStates, String> {
        Err("Not implemented".to_string())
    }

    fn stop(&self, _job: &Job) -> Result<(), String> {
        Err("Not implemented".to_string())
    }

    fn cleanup(&self, _job: &Job) -> Result<(), String> {
        Err("Not implemented".to_string())
    }
}

// Create the use case with mock dependencies
fn create_test_use_case(
    fail_at: Option<&'static str>,
) -> (
    StartJobUseCase<
        MockResourceManager,
        MockRuntimeManager,
        MockCodeRepository,
    >,
    Rc<RefCell<Vec<&'static str>>>,
) {
    let events = Rc::new(RefCell::new(Vec::new()));

    let control = MockControl {
        events: events.clone(),
        fail_at,
    };

    let resource_manager = MockResourceManager {
        control: control.clone(),
    };

    let runtime_manager = MockRuntimeManager {
        control: control.clone(),
    };

    let code_repository = MockCodeRepository {
        control,
    };

    let use_case = StartJobUseCase::new(
        resource_manager,
        runtime_manager,
        code_repository,
    );

    (use_case, events)
}

// Create a valid request
fn create_test_request() -> StartJobRequest {
    StartJobRequest {
        id: "job-001".to_string(),
        name: "test-job".to_string(),
        cpu: 2.0,
        memory: 512.0,
        path: "/tmp/test".to_string(),
        file: vec![1, 2, 3],
        command: "python main.py".to_string(),
    }
}

// Verify a failure and that execution stops at that point
fn assert_failure(
    fail_at: &'static str,
    expected_events: Vec<&'static str>,
) {
    let (use_case, events) = create_test_use_case(Some(fail_at));

    let response = use_case.start_job(create_test_request());

    assert!(response.is_error);
    assert_eq!(response.message, format!("{} failed", fail_at));

    assert_eq!(
        *events.borrow(),
        expected_events
    );
}

#[test]
fn test_start_job_repository_failure() {
    assert_failure(
        "repository.create",
        vec![
            "repository.create",
        ],
    );
}

#[test]
fn test_start_job_resource_configuration_failure() {
    assert_failure(
        "resource.configure",
        vec![
            "repository.create",
            "resource.configure",
        ],
    );
}

#[test]
fn test_start_job_runtime_configuration_failure() {
    assert_failure(
        "runtime.configure",
        vec![
            "repository.create",
            "resource.configure",
            "runtime.configure",
        ],
    );
}

#[test]
fn test_start_job_runtime_start_failure() {
    assert_failure(
        "runtime.start",
        vec![
            "repository.create",
            "resource.configure",
            "runtime.configure",
            "runtime.start",
        ],
    );
}

#[test]
fn test_start_job_get_pid_failure() {
    assert_failure(
        "runtime.get_pid",
        vec![
            "repository.create",
            "resource.configure",
            "runtime.configure",
            "runtime.start",
            "runtime.get_pid",
        ],
    );
}

#[test]
fn test_start_job_resource_start_failure() {
    assert_failure(
        "resource.start",
        vec![
            "repository.create",
            "resource.configure",
            "runtime.configure",
            "runtime.start",
            "runtime.get_pid",
            "resource.start",
        ],
    );
}

#[test]
fn test_start_job_success() {
    let (use_case, events) = create_test_use_case(None);

    let response = use_case.start_job(create_test_request());

    assert!(!response.is_error);

    assert_eq!(
        response.message,
        "Job successfully started!"
    );

    assert_eq!(
        *events.borrow(),
        vec![
            "repository.create",
            "resource.configure",
            "runtime.configure",
            "runtime.start",
            "runtime.get_pid",
            "resource.start",
        ]
    );
}
