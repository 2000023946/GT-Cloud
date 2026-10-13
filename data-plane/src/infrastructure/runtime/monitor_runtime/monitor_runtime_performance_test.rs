
use std::collections::HashMap;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

use crate::{
    domain::{
        app::App,
        job::{Job, JobStatus},
        network::NetworkRule,
        resource::Resource,
        runtime_states::runtime_states::RuntimeStates,
    },
    infrastructure::runtime::{
        monitor_runtime::monitor_runtime::Monitor,
        states::{
            process_state::ProcessState,
            runtime_state::RuntimeState,
        },
    },
    observability::observability::Observability,
    ports::{logger::Logger, metrics::Metrics},
};

struct PerformanceLogger;

impl Logger for PerformanceLogger {
    fn info(&self, _: &str, _: HashMap<String, String>) {}

    fn error(&self, _: &str, _: HashMap<String, String>) {}
}

struct PerformanceMetrics;

impl Metrics for PerformanceMetrics {
    fn increment(&self, _: &str, _: f64) {}

    fn observe(&self, _: &str, _: f64) {}
}

// Simulate a process that is still running.
fn performance_mock_waitpid(
    _: libc::pid_t,
    status: *mut libc::c_int,
    _: libc::c_int,
) -> libc::pid_t {
    unsafe {
        *status = 0;
    }

    0
}

fn make_job(id: &str) -> Job {
    Job {
        id: id.to_string(),
        app: App {
            id: "app-1".to_string(),
            code_path: "/tmp/app".to_string(),
            command: "test".to_string(),
            environment: vec![],
        },
        name: "performance-test".to_string(),
        status: JobStatus::Running,
        resources: Resource {
            cpu: 1,
            memory: 1024,
        },
        network: Vec::<NetworkRule>::new(),
    }
}

fn make_process(pid: Option<u32>) -> ProcessState {
    ProcessState {
        program: "test".to_string(),
        args: vec![],
        working_directory: None,
        environment: vec![],
        pid,
    }
}

// Single-threaded performance test.
#[test]
fn monitor_performance_with_mock_waitpid() {
    const ITERATIONS: u32 = 100_000;

    let state = RuntimeState::new();

    state.add_process(
        "job-1".to_string(),
        make_process(Some(1234)),
    );

    let monitor = Monitor::with_waitpid(
        state,
        Observability {
            logger: PerformanceLogger,
            metrics: PerformanceMetrics,
        },
        performance_mock_waitpid,
    );

    let job = make_job("job-1");

    // Warm up before measuring.
    for _ in 0..1_000 {
        let result = monitor.monitor(job.clone());
        assert!(matches!(result, Ok(RuntimeStates::Running)));
    }

    let start = Instant::now();

    for _ in 0..ITERATIONS {
        let result = monitor.monitor(job.clone());
        assert!(matches!(result, Ok(RuntimeStates::Running)));
    }

    let elapsed = start.elapsed();

    let ns_per_call =
        elapsed.as_nanos() as f64 / ITERATIONS as f64;

    let calls_per_second =
        ITERATIONS as f64 / elapsed.as_secs_f64();

    println!("\nSingle-thread monitor performance:");
    println!("  Iterations:       {ITERATIONS}");
    println!("  Total time:       {elapsed:?}");
    println!("  Average per call: {ns_per_call:.2} ns");
    println!("  Calls per second: {calls_per_second:.0}");
}

// Concurrent performance test using 4, 8, and 16 threads.
#[test]
fn monitor_concurrent_performance_4_8_16_threads() {
    const ITERATIONS_PER_THREAD: u32 = 100_000;

    for thread_count in [4, 8, 16] {
        let state = RuntimeState::new();

        state.add_process(
            "job-1".to_string(),
            make_process(Some(1234)),
        );

        let monitor = Arc::new(Monitor::with_waitpid(
            state,
            Observability {
                logger: PerformanceLogger,
                metrics: PerformanceMetrics,
            },
            performance_mock_waitpid,
        ));

        let job = make_job("job-1");
        let barrier = Arc::new(Barrier::new(thread_count + 1));

        let mut handles = Vec::new();

        for _ in 0..thread_count {
            let monitor = Arc::clone(&monitor);
            let job = job.clone();
            let barrier = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                // Wait until all threads are ready.
                barrier.wait();

                for _ in 0..ITERATIONS_PER_THREAD {
                    let result = monitor.monitor(job.clone());

                    assert!(matches!(
                        result,
                        Ok(RuntimeStates::Running)
                    ));
                }
            }));
        }

        // Release all threads together and start timing.
        let start = Instant::now();
        barrier.wait();

        for handle in handles {
            handle.join().expect("Monitoring thread failed");
        }

        let elapsed = start.elapsed();

        let total_calls =
            thread_count as u64 * ITERATIONS_PER_THREAD as u64;

        let average_us =
            elapsed.as_secs_f64() * 1_000_000.0
                / total_calls as f64;

        let calls_per_second =
            total_calls as f64 / elapsed.as_secs_f64();

        println!("\nConcurrent monitor performance:");
        println!("  Threads:          {thread_count}");
        println!("  Total calls:      {total_calls}");
        println!("  Total time:       {elapsed:?}");
        println!("  Average per call: {average_us:.2} us");
        println!("  Throughput:       {calls_per_second:.0} calls/second");
    }
}
