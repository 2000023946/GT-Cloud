#[cfg(test)]
mod concurrency_tests {
    use crate::{
        domain::{
            app::App,
            job::{Job, JobStatus},
            network::NetworkRule,
            resource::Resource,
        },
        infrastructure::runtime::{
            cleanup::{
                cleanup_runtime::CleanupRuntime,
                cleanup_runtime_telemetry::CleanupRuntimeTelemetry,
            },
            states::{
                process_state::ProcessState,
                runtime_state::RuntimeState,
            },
        },
        observability::observability::Observability,
        ports::{logger::Logger, metrics::Metrics},
    };

    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        thread,
    };

    // --------------------------------------------------
    // Test Logger
    // --------------------------------------------------

    #[derive(Clone, Default)]
    struct TestLogger {
        info_calls: Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
        error_calls: Arc<Mutex<Vec<(String, HashMap<String, String>)>>>,
    }

    impl Logger for TestLogger {
        fn info(&self, message: &str, fields: HashMap<String, String>) {
            self.info_calls
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }

        fn error(&self, message: &str, fields: HashMap<String, String>) {
            self.error_calls
                .lock()
                .unwrap()
                .push((message.to_string(), fields));
        }
    }

    // --------------------------------------------------
    // Test Metrics
    // --------------------------------------------------

    #[derive(Clone, Default)]
    struct TestMetrics {
        increment_calls: Arc<Mutex<Vec<(String, f64)>>>,
        observe_calls: Arc<Mutex<Vec<(String, f64)>>>,
    }

    impl Metrics for TestMetrics {
        fn increment(&self, name: &str, value: f64) {
            self.increment_calls
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }

        fn observe(&self, name: &str, value: f64) {
            self.observe_calls
                .lock()
                .unwrap()
                .push((name.to_string(), value));
        }
    }

    // --------------------------------------------------
    // Test Helpers
    // --------------------------------------------------

    fn create_job(job_id: &str) -> Job {
        Job {
            id: job_id.to_string(),
            app: App {
                id: "app-123".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "python app.py".to_string(),
                environment: vec![],
            },
            name: "test-job".to_string(),
            status: JobStatus::Stopped,
            resources: Resource {
                cpu: 1,
                memory: 512,
            },
            network: Vec::<NetworkRule>::new(),
        }
    }

    fn create_cleanup_runtime() -> (
        CleanupRuntime<TestLogger, TestMetrics>,
        Arc<RuntimeState>,
        TestLogger,
        TestMetrics,
    ) {
        let runtime_state = Arc::new(RuntimeState::new());

        let logger = TestLogger::default();
        let metrics = TestMetrics::default();

        let observability = Observability {
            logger: logger.clone(),
            metrics: metrics.clone(),
        };

        let telemetry = CleanupRuntimeTelemetry::new(observability);

        let cleanup = CleanupRuntime::new(telemetry);

        (cleanup, runtime_state, logger, metrics)
    }

    // --------------------------------------------------
    // Concurrency Tests
    // --------------------------------------------------

    #[test]
    fn cleanup_handles_concurrent_different_jobs() {
        let (cleanup, runtime_state, _, _) = create_cleanup_runtime();

        // Prepare 100 jobs.
        for i in 0..100 {
            let job_id = format!("job-{}", i);

            runtime_state.add_process(
                job_id.clone(),
                ProcessState {
                    program: "python".to_string(),
                    args: vec!["app.py".to_string()],
                    working_directory: None,
                    environment: vec![],
                    pid: Some(i + 1),
                },
            );

            runtime_state.add_stack(job_id, vec![0u8; 4096]);
        }

        // Clean up all jobs concurrently.
        thread::scope(|scope| {
            let mut handles = Vec::new();

            for i in 0..100 {
                let job_id = format!("job-{}", i);
                let cleanup = &cleanup;
                let runtime_state = &runtime_state;

                handles.push(scope.spawn(move || {
                    cleanup
                        .cleanup(runtime_state, create_job(&job_id))
                        .unwrap();
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });

        // Verify all process and stack state was removed.
        assert_eq!(runtime_state.process_count(), 0);
        assert_eq!(runtime_state.stack_count(), 0);
    }

    #[test]
    fn cleanup_handles_concurrent_same_job() {
        let (cleanup, runtime_state, _, _) = create_cleanup_runtime();

        let job_id = "shared-job";

        runtime_state.add_process(
            job_id.to_string(),
            ProcessState {
                program: "python".to_string(),
                args: vec!["app.py".to_string()],
                working_directory: None,
                environment: vec![],
                pid: Some(1234),
            },
        );

        runtime_state.add_stack(
            job_id.to_string(),
            vec![0u8; 4096],
        );

        // Multiple threads attempt to clean up the same job.
        thread::scope(|scope| {
            let mut handles = Vec::new();

            for _ in 0..20 {
                let cleanup = &cleanup;
                let runtime_state = &runtime_state;

                handles.push(scope.spawn(move || {
                    cleanup
                        .cleanup(runtime_state, create_job(job_id))
                        .unwrap();
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });

        // Verify the shared job was cleaned up.
        assert!(!runtime_state.contains_process(job_id));
        assert!(!runtime_state.contains_stack(job_id));
    }
}


#[cfg(test)]
mod performance_tests {
    use crate::{
        domain::{
            app::App,
            job::{Job, JobStatus},
            network::NetworkRule,
            resource::Resource,
        },
        infrastructure::runtime::{
            cleanup::{
                cleanup_runtime::CleanupRuntime,
                cleanup_runtime_telemetry::CleanupRuntimeTelemetry,
            },
            states::{
                process_state::ProcessState,
                runtime_state::RuntimeState,
            },
        },
        observability::observability::Observability,
        ports::{logger::Logger, metrics::Metrics},
    };

    use std::{
        collections::HashMap,
        sync::Arc,
        thread,
        time::Instant,
    };

    #[derive(Clone, Default)]
    struct TestLogger;

    impl Logger for TestLogger {
        fn info(&self, _: &str, _: HashMap<String, String>) {}
        fn error(&self, _: &str, _: HashMap<String, String>) {}
    }

    #[derive(Clone, Default)]
    struct TestMetrics;

    impl Metrics for TestMetrics {
        fn increment(&self, _: &str, _: f64) {}
        fn observe(&self, _: &str, _: f64) {}
    }

    fn create_job(job_id: String) -> Job {
        Job {
            id: job_id,
            app: App {
                id: "app-123".to_string(),
                code_path: "/tmp/app".to_string(),
                command: "python app.py".to_string(),
                environment: vec![],
            },
            name: "test-job".to_string(),
            status: JobStatus::Stopped,
            resources: Resource {
                cpu: 1,
                memory: 512,
            },
            network: Vec::<NetworkRule>::new(),
        }
    }

    fn create_cleanup_runtime() -> (
        CleanupRuntime<TestLogger, TestMetrics>,
        Arc<RuntimeState>,
    ) {
        let state = Arc::new(RuntimeState::new());

        let observability = Observability {
            logger: TestLogger,
            metrics: TestMetrics,
        };

        let telemetry = CleanupRuntimeTelemetry::new(observability);
        let cleanup = CleanupRuntime::new(telemetry);

        (cleanup, state)
    }

    fn prepare_jobs(state: &RuntimeState, count: usize) {
        for i in 0..count {
            let job_id = format!("job-{i}");

            state.add_process(
                job_id.clone(),
                ProcessState {
                    program: "python".to_string(),
                    args: vec!["app.py".to_string()],
                    working_directory: None,
                    environment: vec![],
                    pid: Some(i as u32 + 1),
                },
            );

            state.add_stack(job_id, vec![0u8; 4096]);
        }
    }

    fn print_results(
        label: &str,
        operations: usize,
        elapsed: std::time::Duration,
    ) {
        let seconds = elapsed.as_secs_f64();
        let throughput = operations as f64 / seconds;
        let average = seconds * 1_000_000.0 / operations as f64;

        println!("\n========== {label} ==========");
        println!("Operations:       {operations}");
        println!("Total time:       {:.3} ms", seconds * 1000.0);
        println!("Throughput:       {:.0} cleanups/sec", throughput);
        println!("Average cleanup:  {:.3} us", average);
        println!("================================\n");
    }

    #[test]
    fn cleanup_single_thread_performance() {
        let operations = 100_000;
        let (cleanup, state) = create_cleanup_runtime();

        prepare_jobs(&state, operations);

        let start = Instant::now();

        for i in 0..operations {
            cleanup
                .cleanup(&state, create_job(format!("job-{i}")))
                .unwrap();
        }

        let elapsed = start.elapsed();

        assert_eq!(state.process_count(), 0);
        assert_eq!(state.stack_count(), 0);

        print_results("Single-thread cleanup", operations, elapsed);
    }

    #[test]
    fn cleanup_multi_thread_performance() {
        let operations_per_thread = 25_000;

        for thread_count in [1, 4, 8, 16] {
            let total_operations = operations_per_thread * thread_count;

            let (cleanup, state) = create_cleanup_runtime();

            prepare_jobs(&state, total_operations);

            let start = Instant::now();

            thread::scope(|scope| {
                let mut handles = Vec::new();

                for thread_id in 0..thread_count {
                    let cleanup = &cleanup;
                    let state = &state;

                    handles.push(scope.spawn(move || {
                        for i in 0..operations_per_thread {
                            let job_id = format!(
                                "job-{}",
                                thread_id * operations_per_thread + i
                            );

                            cleanup
                                .cleanup(state, create_job(job_id))
                                .unwrap();
                        }
                    }));
                }

                for handle in handles {
                    handle.join().unwrap();
                }
            });

            let elapsed = start.elapsed();

            assert_eq!(state.process_count(), 0);
            assert_eq!(state.stack_count(), 0);

            print_results(
                &format!("{thread_count}-thread cleanup"),
                total_operations,
                elapsed,
            );
        }
    }
}