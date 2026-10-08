


#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        process::Command,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
        thread,
        time::{Duration, Instant},
    };

    use crate::{
        domain::{
            app::app::App,
            job::{
                job::Job,
                job_status::JobStatus,
            },
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

    fn create_job(index: usize) -> Job {
        Job {
            id: format!("benchmark-job-{index}"),
            app: App {
                id: format!("benchmark-app-{index}"),
                code_path: "/tmp/app".to_string(),
                command: "/bin/sleep".to_string(),
                environment: vec![],
            },
            name: format!("benchmark-{index}"),
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

    fn create_configurer() -> ConfigureRuntime<TestLogger, TestMetrics> {
        ConfigureRuntime {
            working_directory_manager: WorkingDirectoryManager {
                root: "/tmp/gt-cloud-benchmark".to_string(),
            },
            observability: Observability {
                logger: TestLogger,
                metrics: TestMetrics,
            },
        }
    }

    fn percentile(samples: &[Duration], p: usize) -> Duration {
        if samples.is_empty() {
            return Duration::ZERO;
        }

        let mut sorted = samples.to_vec();
        sorted.sort_unstable();

        let index = ((p * sorted.len() + 99) / 100)
            .saturating_sub(1)
            .min(sorted.len() - 1);

        sorted[index]
    }

    fn milliseconds(duration: Duration) -> f64 {
        duration.as_secs_f64() * 1000.0
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn start_runtime_performance_test() {
        const THREAD_COUNTS: [usize; 3] = [4, 8, 16];
        const WORKLOAD_COUNTS: [usize; 3] = [1_000, 10_000, 100_000];

        let runtime = Arc::new(create_runtime());
        let configurer = Arc::new(create_configurer());

        println!("\n==========================================");
        println!("   StartRuntime Performance Benchmark");
        println!("==========================================");
        println!("Platform: Linux");
        println!("Workloads: 1,000 / 10,000 / 100,000");
        println!("Concurrency: 4 / 8 / 16 threads\n");

        println!(
            "{:>8} {:>12} {:>10} {:>10} {:>13} {:>11} {:>10} {:>10} {:>10}",
            "Threads", "Workloads", "Success", "Failures",
            "Starts/sec", "Avg(ms)", "p50(ms)", "p95(ms)", "p99(ms)"
        );

        for workloads in WORKLOAD_COUNTS {
            for threads in THREAD_COUNTS {
                let next = Arc::new(AtomicUsize::new(0));
                let latencies = Arc::new(Mutex::new(Vec::<Duration>::new()));
                let successes = Arc::new(AtomicUsize::new(0));
                let failures = Arc::new(AtomicUsize::new(0));

                let wall_start = Instant::now();
                let mut handles = Vec::new();

                for _ in 0..threads {
                    let runtime = Arc::clone(&runtime);
                    let configurer = Arc::clone(&configurer);
                    let next = Arc::clone(&next);
                    let latencies = Arc::clone(&latencies);
                    let successes = Arc::clone(&successes);
                    let failures = Arc::clone(&failures);

                    handles.push(thread::spawn(move || loop {
                        let index = next.fetch_add(1, Ordering::Relaxed);

                        if index >= workloads {
                            break;
                        }

                        let job = create_job(index);
                        let state = RuntimeState::new();

                        // Prepare the process state before measuring startup.
                        if let Err(error) = configurer.configure(&state, job.clone()) {
                            failures.fetch_add(1, Ordering::Relaxed);
                            eprintln!(
                                "Configure failed for {}: {}",
                                job.id, error
                            );
                            continue;
                        }

                        // Measure the actual StartRuntime::start call.
                        let start = Instant::now();
                        let result = runtime.start(&state, job.clone());
                        let latency = start.elapsed();

                        latencies
                            .lock()
                            .expect("latency mutex poisoned")
                            .push(latency);

                        match result {
                            Ok(()) => {
                                successes.fetch_add(1, Ordering::Relaxed);

                                // Stop the process before this worker starts
                                // another job, limiting active jobs by threads.
                                if let Some(process_state) =
                                    state.get_process(&job.id)
                                {
                                    if let Some(pid) = process_state.pid {
                                        let _ = Command::new("kill")
                                            .arg("-KILL")
                                            .arg(pid.to_string())
                                            .status();
                                    }
                                }
                            }
                            Err(error) => {
                                failures.fetch_add(1, Ordering::Relaxed);
                                eprintln!(
                                    "Start failed for {}: {}",
                                    job.id, error
                                );
                            }
                        }
                    }));
                }

                for handle in handles {
                    if handle.join().is_err() {
                        failures.fetch_add(1, Ordering::Relaxed);
                        eprintln!("A benchmark worker panicked");
                    }
                }

                let elapsed = wall_start.elapsed();
                let success_count = successes.load(Ordering::Relaxed);
                let failure_count = failures.load(Ordering::Relaxed);

                let samples = latencies
                    .lock()
                    .expect("latency mutex poisoned")
                    .clone();

                let average = if samples.is_empty() {
                    Duration::ZERO
                } else {
                    Duration::from_nanos(
                        (samples.iter().map(Duration::as_nanos).sum::<u128>()
                            / samples.len() as u128)
                            .min(u64::MAX as u128) as u64,
                    )
                };

                let throughput = if elapsed.is_zero() {
                    0.0
                } else {
                    success_count as f64 / elapsed.as_secs_f64()
                };

                println!(
                    "{:>8} {:>12} {:>10} {:>10} {:>13.2} {:>11.3} \
                     {:>10.3} {:>10.3} {:>10.3}",
                    threads,
                    workloads,
                    success_count,
                    failure_count,
                    throughput,
                    milliseconds(average),
                    milliseconds(percentile(&samples, 50)),
                    milliseconds(percentile(&samples, 95)),
                    milliseconds(percentile(&samples, 99)),
                );
            }
        }

        println!("\nBenchmark finished.");
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn start_runtime_performance_test() {
        println!(
            "Actual StartRuntime performance benchmark requires Linux. \
             Run this test inside your Linux Docker container."
        );
    }
}