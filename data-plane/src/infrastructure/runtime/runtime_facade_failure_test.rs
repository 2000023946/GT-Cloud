
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;

    use crate::domain::{
        app::App,
        job::{Job, JobStatus},
        resource::Resource,
        runtime_states::runtime_states::RuntimeStates,
    };

    use crate::infrastructure::runtime::runtime_facade::RuntimeFacade;
    use crate::ports::logger::Logger;
    use crate::ports::metrics::Metrics;

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
            id: "failure-test-001".to_string(),
            name: "External Process Termination Test".to_string(),
            app: App {
                id: "failure-test-app".to_string(),
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
    fn test_runtime_handles_external_process_termination() {
        let facade = RuntimeFacade::new(TestLogger, TestMetrics);
        let job = create_test_job();

        // 1. Configure the job.
        facade
            .configure(job.clone())
            .expect("Job configuration failed");

        println!("Job configured");

        // 2. Start the job.
        facade
            .start(job.clone())
            .expect("Job start failed");

        println!("Job started");

        // 3. Retrieve the process PID.
        let process = facade
            .runtime_state()
            .get_process(&job.id)
            .expect("Process state not found");

        let pid = process.pid.expect("Process PID not found");

        println!("Running process PID: {}", pid);

        // 4. Kill the process externally using SIGKILL.
        let kill_result = unsafe {
            libc::kill(pid as libc::pid_t, libc::SIGKILL)
        };

        assert_eq!(
            kill_result,
            0,
            "Failed to externally terminate process"
        );

        println!("SIGKILL sent to process");

        // 5. Poll Monitor until process termination is detected.
        let mut status = RuntimeStates::Running;

        for attempt in 1..=20 {
            status = facade
                .monitor(job.clone())
                .expect("Monitoring failed");

            if matches!(
                status,
                RuntimeStates::Signaled { signal }
                    if signal == libc::SIGKILL
            ) {
                println!(
                    "Monitor detected SIGKILL on attempt {}",
                    attempt
                );
                break;
            }

            thread::sleep(Duration::from_millis(50));
        }

        assert!(
            matches!(
                status,
                RuntimeStates::Signaled { signal }
                    if signal == libc::SIGKILL
            ),
            "Expected process to be detected as terminated by SIGKILL"
        );

        // 6. Verify that the terminal status was saved.
        let saved_status = facade
            .runtime_state()
            .get_status(&job.id)
            .expect("Runtime status was not saved");

        assert!(
            matches!(
                saved_status,
                RuntimeStates::Signaled { signal }
                    if signal == libc::SIGKILL
            ),
            "Expected SIGKILL status to be saved"
        );

        println!("Terminal process status saved");

        // 7. Clean up the terminated job.
        facade
            .cleanup(job.clone())
            .expect("Job cleanup failed");

        println!("Job cleanup completed");

        // 8. Verify that process and stack state were removed.
        assert!(
            !facade.runtime_state().contains_process(&job.id),
            "Process state should have been removed"
        );

        assert!(
            !facade.runtime_state().contains_stack(&job.id),
            "Child stack should have been removed"
        );

        println!("Process and stack state successfully removed");
    }
}
