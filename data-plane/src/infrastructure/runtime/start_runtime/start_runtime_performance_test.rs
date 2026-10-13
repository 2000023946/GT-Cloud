
use std::{
    collections::HashMap,
    fs,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Barrier, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::{
    domain::{
        app::app::App,
        job::{job::Job, job_status::JobStatus},
        resource::resource::Resource,
    },
    infrastructure::runtime::{
        configure_runtime::configure_runtime::ConfigureRuntime,
        helpers::working_directory_manager::WorkingDirectoryManager,
        start_runtime::start_runtime::StartRuntime,
        states::runtime_state::RuntimeState,
    },
    observability::observability::Observability,
    ports::{logger::Logger, metrics::Metrics},
};

struct TestLogger;

impl Logger for TestLogger {
    fn info(&self, _: &str, _: HashMap<String, String>) {}
    fn error(&self, _: &str, _: HashMap<String, String>) {}
}

struct TestMetrics;

impl Metrics for TestMetrics {
    fn increment(&self, _: &str, _: f64) {}
    fn observe(&self, _: &str, _: f64) {}
}

fn create_job(id: String) -> Job {
    Job {
        id: id.clone(),
        app: App {
            id: format!("app-{id}"),
            code_path: "/tmp/app".to_string(),
            command: "/bin/true".to_string(),
            environment: vec![],
        },
        name: id,
        status: JobStatus::Received,
        resources: Resource {
            cpu: 1,
            memory: 128,
        },
        network: vec![],
    }
}

fn create_runtime() -> StartRuntime<TestLogger, TestMetrics> {
    StartRuntime {
        observability: Observability {
            logger: TestLogger,
            metrics: TestMetrics,
        },
    }
}

fn create_configure_runtime() -> ConfigureRuntime<TestLogger, TestMetrics> {
    ConfigureRuntime {
        working_directory_manager: WorkingDirectoryManager {
            root: std::env::temp_dir()
                .join("gt-cloud-performance-tests")
                .to_string_lossy()
                .into_owned(),
        },
        observability: Observability {
            logger: TestLogger,
            metrics: TestMetrics,
        },
    }
}

#[cfg(target_os = "linux")]
fn wait_for_child(pid: u32) {
    let mut status: libc::c_int = 0;

    loop {
        let result = unsafe {
            libc::waitpid(pid as libc::pid_t, &mut status, 0)
        };

        if result >= 0 {
            break;
        }

        let error = std::io::Error::last_os_error();

        if error.kind() != std::io::ErrorKind::Interrupted {
            // The runtime may already have reaped the child.
            break;
        }
    }
}

#[cfg(target_os = "linux")]
fn run_one_job(
    concurrency: usize,
    repetition: usize,
    index: usize,
) -> Result<f64, String> {
    let job_id = format!(
        "perf-{}-{}-{}-{}",
        std::process::id(),
        concurrency,
        repetition,
        index
    );

    let state = RuntimeState::new();
    let job = create_job(job_id.clone());

    create_configure_runtime()
        .configure(&state, job.clone())
        .map_err(|error| format!("Configuration failed: {error:?}"))?;

    let runtime = create_runtime();

    // Measure only the runtime startup operation.
    let start = Instant::now();
    let result = runtime.start(&state, job.clone());
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    let process = state.get_process(&job.id);
    let pid = process.as_ref().and_then(|process| process.pid);
    let working_directory = process
        .and_then(|process| process.working_directory.clone());

    // Reap the child before deleting its output directory.
    if let Some(pid) = pid {
        wait_for_child(pid);
    }

    if let Some(directory) = working_directory {
        let _ = fs::remove_dir_all(directory);
    }

    result
        .map(|()| latency_ms)
        .map_err(|error| format!("Startup failed: {error:?}"))
}

#[cfg(target_os = "linux")]
fn print_results(
    title: &str,
    latencies: &mut [f64],
    successes: usize,
    failures: usize,
    elapsed: Duration,
) {
    latencies.sort_by(f64::total_cmp);

    let average_ms = if latencies.is_empty() {
        0.0
    } else {
        latencies.iter().sum::<f64>() / latencies.len() as f64
    };

    let percentile_ms = |percentile: usize| -> f64 {
        if latencies.is_empty() {
            return 0.0;
        }

        let index = (percentile * latencies.len()).div_ceil(100);
        latencies[index.saturating_sub(1).min(latencies.len() - 1)]
    };

    let throughput = if elapsed.as_secs_f64() > 0.0 {
        successes as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };

    let total = successes + failures;
    let failure_rate = if total > 0 {
        failures as f64 / total as f64 * 100.0
    } else {
        0.0
    };

    println!("\n========== {title} ==========");
    println!("Successful starts:  {successes}");
    println!("Failed attempts:    {failures}");
    println!("Failure rate:       {failure_rate:.2}%");
    println!("Average startup:    {average_ms:.3} ms");
    println!("p50 startup:        {:.3} ms", percentile_ms(50));
    println!("p95 startup:        {:.3} ms", percentile_ms(95));
    println!("p99 startup:        {:.3} ms", percentile_ms(99));
    println!("Throughput:         {throughput:.2} starts/sec");
    println!("Elapsed time:       {:.3} sec", elapsed.as_secs_f64());
}

#[cfg(target_os = "linux")]
#[test]
fn start_runtime_performance_test() {
    const CONCURRENCY_LEVELS: [usize; 4] = [1, 4, 8, 16];
    const BURST_REPETITIONS: usize = 3;

    println!("\n========== GT CLOUD PERFORMANCE TESTS ==========");

    // ---------------------------------------------------------
    // Test 1: Burst concurrency
    // ---------------------------------------------------------

    println!("\nTest 1: Concurrent startup bursts");

    for concurrency in CONCURRENCY_LEVELS {
        let mut latencies = Vec::new();
        let mut successes = 0;
        let mut failures = 0;
        let mut elapsed = Duration::ZERO;

        for repetition in 0..BURST_REPETITIONS {
            let barrier = Arc::new(Barrier::new(concurrency + 1));
            let mut handles = Vec::with_capacity(concurrency);

            for index in 0..concurrency {
                let barrier = Arc::clone(&barrier);

                handles.push(thread::spawn(move || {
                    // Prepare the job before synchronizing the startup.
                    let job_id = format!(
                        "burst-{}-{}-{}-{}",
                        std::process::id(),
                        concurrency,
                        repetition,
                        index
                    );

                    let state = RuntimeState::new();
                    let job = create_job(job_id);

                    let configure_result =
                        create_configure_runtime().configure(&state, job.clone());

                    if let Err(error) = configure_result {
                        barrier.wait();
                        return Err(format!("Configuration failed: {error:?}"));
                    }

                    let runtime = create_runtime();
                    barrier.wait();

                    let start = Instant::now();
                    let result = runtime.start(&state, job.clone());
                    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

                    let process = state.get_process(&job.id);
                    let pid = process.as_ref().and_then(|process| process.pid);
                    let directory = process
                        .and_then(|process| process.working_directory.clone());

                    if let Some(pid) = pid {
                        wait_for_child(pid);
                    }

                    if let Some(directory) = directory {
                        let _ = fs::remove_dir_all(directory);
                    }

                    result
                        .map(|()| latency_ms)
                        .map_err(|error| format!("Startup failed: {error:?}"))
                }));
            }

            let batch_start = Instant::now();
            barrier.wait();

            for handle in handles {
                match handle.join() {
                    Ok(Ok(latency)) => {
                        latencies.push(latency);
                        successes += 1;
                    }
                    Ok(Err(error)) => {
                        failures += 1;
                        eprintln!("{error}");
                    }
                    Err(_) => {
                        failures += 1;
                        eprintln!("Benchmark thread panicked");
                    }
                }
            }

            elapsed += batch_start.elapsed();
        }

        print_results(
            &format!("BURST: {concurrency} concurrent starts"),
            &mut latencies,
            successes,
            failures,
            elapsed,
        );
    }

    // ---------------------------------------------------------
    // Test 2: Sustained load
    // ---------------------------------------------------------

    println!("\nTest 2: Sustained startup load");

    const LOAD_DURATION: Duration = Duration::from_secs(10);

    // Safety limit prevents an unexpected runtime or container
    // configuration from creating unlimited processes.
    const MAX_ATTEMPTS_PER_LEVEL: usize = 500;

    for concurrency in CONCURRENCY_LEVELS {
        let attempts = Arc::new(AtomicUsize::new(0));
        let successes = Arc::new(AtomicUsize::new(0));
        let failures = Arc::new(AtomicUsize::new(0));
        let latencies = Arc::new(Mutex::new(Vec::<f64>::new()));

        let barrier = Arc::new(Barrier::new(concurrency + 1));
        let mut handles = Vec::with_capacity(concurrency);

        for worker in 0..concurrency {
            let attempts = Arc::clone(&attempts);
            let successes = Arc::clone(&successes);
            let failures = Arc::clone(&failures);
            let latencies = Arc::clone(&latencies);
            let barrier = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier.wait();

                let deadline = Instant::now() + LOAD_DURATION;

                loop {
                    if Instant::now() >= deadline {
                        break;
                    }

                    // Atomically reserve one attempt, respecting the cap.
                    let reserved = attempts.fetch_update(
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                        |current| {
                            if current < MAX_ATTEMPTS_PER_LEVEL {
                                Some(current + 1)
                            } else {
                                None
                            }
                        },
                    );

                    if reserved.is_err() {
                        break;
                    }

                    let index = reserved.unwrap();

                    match run_one_job(concurrency, worker, index) {
                        Ok(latency_ms) => {
                            successes.fetch_add(1, Ordering::Relaxed);

                            if let Ok(mut values) = latencies.lock() {
                                values.push(latency_ms);
                            }
                        }
                        Err(error) => {
                            failures.fetch_add(1, Ordering::Relaxed);
                            eprintln!("{error}");
                        }
                    }
                }
            }));
        }

        let load_start = Instant::now();
        barrier.wait();

        for handle in handles {
            if handle.join().is_err() {
                failures.fetch_add(1, Ordering::Relaxed);
                eprintln!("Sustained-load worker panicked");
            }
        }

        let elapsed = load_start.elapsed();
        let successes = successes.load(Ordering::Relaxed);
        let failures = failures.load(Ordering::Relaxed);

        let mut latency_values = match latencies.lock() {
            Ok(values) => values.clone(),
            Err(_) => Vec::new(),
        };

        print_results(
            &format!("SUSTAINED LOAD: {concurrency} workers"),
            &mut latency_values,
            successes,
            failures,
            elapsed,
        );

        println!(
            "Attempt cap reached: {}",
            attempts.load(Ordering::Relaxed) >= MAX_ATTEMPTS_PER_LEVEL
        );
    }

    println!("\n========== ALL PERFORMANCE TESTS COMPLETE ==========");
}