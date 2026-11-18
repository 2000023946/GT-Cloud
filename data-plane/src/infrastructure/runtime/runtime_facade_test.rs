#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::domain::{
        app::App,
        job::{Job, JobStatus},
        resource::Resource,
    };

    use crate::ports::logger::Logger;
    use crate::ports::metrics::Metrics;
    use crate::infrastructure::runtime::runtime_facade::RuntimeFacade;
    use crate::domain::runtime_states::runtime_states::RuntimeStates;

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
                cpu: 1,
                memory: 128 * 1024 * 1024,
            },

            network: vec![],
        }
    }

    #[test]
    fn test_complete_runtime_lifecycle() {
        let facade = RuntimeFacade::new(
            TestLogger,
            TestMetrics,
        );

        let job = create_test_job();

        // 1. Configure
        facade.configure(job.clone())
            .expect("Configuration failed");

        println!("Job configured");

        // 2. Start
        facade.start(job.clone())
            .expect("Starting job failed");

        println!("Job started");

        // 3. Monitor
        let status = facade.monitor(job.clone())
            .expect("Monitoring failed");

        assert!(
            matches!(status, RuntimeStates::Running),
            "Expected job to be running"
        );

        println!("Job is running");

        // 4. Stop
        facade.stop(job.clone())
            .expect("Stopping job failed");

        println!("Job stopped");

        // 5. Cleanup
        facade.cleanup(job.clone())
            .expect("Cleanup failed");

        println!("Job cleaned up");
    }
}