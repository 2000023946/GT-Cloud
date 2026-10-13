use crate::{
        domain::{
            app::App, job::{Job, JobStatus}, resource::Resource,
        }, infrastructure::runtime::{
            states::{process_state::ProcessState, runtime_state::RuntimeState}, stop_runtime::{stop_runtime::StopRuntime, stop_runtime_os_adapter::StopRuntimeOsAdapter, stop_runtime_telemetry::StopRuntimeTelemetry},
        }, observability::observability::Observability, ports::{logger::Logger, metrics::Metrics},
    };
use std::collections::HashMap;
fn make_job() -> Job {
    Job {
        id: "job-1".to_string(),
        app: App {
            id: "app-1".to_string(),
            code_path: "/tmp/test".to_string(),
            command: "test-program".to_string(),
            environment: vec![],
        },
        name: "test-job".to_string(),
        status: JobStatus::Running,
        resources: Resource {
            cpu: 1,
            memory: 1024,
        },
        network: vec![],
    }
}
    
#[test]
fn performance_stop_runtime_100k_calls_at_1_4_8_16_threads() {
    use std::{
        io,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Barrier,
        },
        thread,
        time::Instant,
    };

    // Lightweight mocks: count calls without storing 100K records.
    #[derive(Clone, Default)]
    struct PerfLogger {
        info_calls: Arc<AtomicUsize>,
        error_calls: Arc<AtomicUsize>,
    }

    impl Logger for PerfLogger {
        fn info(
            &self,
            _message: &str,
            _fields: HashMap<String, String>,
        ) {
            self.info_calls.fetch_add(1, Ordering::Relaxed);
        }

        fn error(
            &self,
            _message: &str,
            _fields: HashMap<String, String>,
        ) {
            self.error_calls.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[derive(Clone, Default)]
    struct PerfMetrics {
        increments: Arc<AtomicUsize>,
    }

    impl Metrics for PerfMetrics {
        fn increment(&self, _name: &str, _value: f64) {
            self.increments.fetch_add(1, Ordering::Relaxed);
        }

        fn observe(&self, _name: &str, _value: f64) {}
    }

    #[derive(Clone, Default)]
    struct PerfStopRuntimeOsAdapter {
        kill_calls: Arc<AtomicUsize>,
    }

    impl StopRuntimeOsAdapter for PerfStopRuntimeOsAdapter {
        fn kill(
            &self,
            _pid: libc::pid_t,
            _signal: libc::c_int,
        ) -> libc::c_int {
            self.kill_calls.fetch_add(1, Ordering::Relaxed);
            0 // Simulate successful SIGTERM.
        }

        fn last_os_error(&self) -> io::Error {
            io::Error::other("Unexpected mock OS error")
        }
    }

    const TOTAL_CALLS: usize = 100_000;

    for concurrency in [1, 4, 8, 16] {
        let runtime_state = RuntimeState::new();

        runtime_state.add_process(
            "job-1".to_string(),
            ProcessState {
                program: "test-program".to_string(),
                args: vec![],
                working_directory: None,
                environment: vec![],
                pid: Some(1234),
            },
        );

        let logger = PerfLogger::default();
        let metrics = PerfMetrics::default();
        let adapter = PerfStopRuntimeOsAdapter::default();

        let telemetry = StopRuntimeTelemetry::new(Observability {
            logger: logger.clone(),
            metrics: metrics.clone(),
        });

        let runtime = Arc::new(StopRuntime::new(
            runtime_state,
            telemetry,
            adapter.clone(),
        ));

        // Prepare jobs before timing to reduce setup overhead.
        let calls_per_thread = TOTAL_CALLS / concurrency;
        let barrier = Arc::new(Barrier::new(concurrency + 1));
        let mut handles = Vec::with_capacity(concurrency);

        for _ in 0..concurrency {
            let runtime = Arc::clone(&runtime);
            let barrier = Arc::clone(&barrier);

            let jobs = vec![make_job(); calls_per_thread];

            handles.push(thread::spawn(move || {
                barrier.wait();

                for job in jobs {
                    runtime
                        .stop(job)
                        .expect("mock stop operation should succeed");
                }
            }));
        }

        // Release all worker threads together.
        let start = Instant::now();
        barrier.wait();

        for handle in handles {
            handle.join().expect("performance worker panicked");
        }

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();
        let throughput = TOTAL_CALLS as f64 / seconds;

        // Verify every operation completed successfully.
        assert_eq!(
            adapter.kill_calls.load(Ordering::Relaxed),
            TOTAL_CALLS
        );
        assert_eq!(
            logger.info_calls.load(Ordering::Relaxed),
            TOTAL_CALLS
        );
        assert_eq!(
            logger.error_calls.load(Ordering::Relaxed),
            0
        );
        assert_eq!(
            metrics.increments.load(Ordering::Relaxed),
            TOTAL_CALLS
        );

        println!(
            "StopRuntime performance: threads={concurrency}, \
             calls={TOTAL_CALLS}, elapsed={:.3}s, \
             throughput={:.0} calls/sec, avg={:.2} us/call",
            seconds,
            throughput,
            seconds * 1_000_000.0 / TOTAL_CALLS as f64,
        );
    }
}