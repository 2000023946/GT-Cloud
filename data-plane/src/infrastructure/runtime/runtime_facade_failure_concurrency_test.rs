
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Barrier};
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

    fn create_test_job(id: usize) -> Job {
        let job_id = format!("random-kill-job-{id}");

        Job {
            id: job_id.clone(),
            name: format!("Random Kill Test Job {id}"),
            app: App {
                id: format!("test-app-{id}"),
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

    // Seeded Fisher-Yates shuffle.
    // The same seed produces the same selection every run.
    fn select_random_kills(job_count: usize, kill_count: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..job_count).collect();
        let mut state: u64 = 0xA5A5_1234_9876_FEDC;

        for i in (1..indices.len()).rev() {
            // Xorshift64 pseudo-random number generator.
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;

            let j = (state as usize) % (i + 1);
            indices.swap(i, j);
        }

        indices.truncate(kill_count);
        indices
    }

    #[test]
    fn test_concurrent_jobs_with_random_external_kills() {
        const JOB_COUNT: usize = 10;
        const KILL_COUNT: usize = 4;
        const MAX_MONITOR_ATTEMPTS: usize = 20;

        let facade = Arc::new(RuntimeFacade::new(TestLogger, TestMetrics));

        let jobs: Vec<Job> = (0..JOB_COUNT).map(create_test_job).collect();

        // 1. Select jobs that will receive external SIGKILL.
        let killed_indices = select_random_kills(JOB_COUNT, KILL_COUNT);

        println!("Jobs selected for SIGKILL: {:?}", killed_indices);

        // 2. Configure and start all jobs concurrently.
        let start_barrier = Arc::new(Barrier::new(JOB_COUNT));

        let start_handles: Vec<_> = jobs
            .iter()
            .cloned()
            .map(|job| {
                let facade = Arc::clone(&facade);
                let barrier = Arc::clone(&start_barrier);

                thread::spawn(move || {
                    facade
                        .configure(job.clone())
                        .expect("Concurrent job configuration failed");

                    barrier.wait();

                    facade
                        .start(job.clone())
                        .expect("Concurrent job start failed");

                    job
                })
            })
            .collect();

        let started_jobs: Vec<Job> = start_handles
            .into_iter()
            .map(|handle| handle.join().expect("Job start thread panicked"))
            .collect();

        println!("All {} jobs started", JOB_COUNT);

        // 3. Verify that all jobs are running before introducing failures.
        for job in &started_jobs {
            let status = facade
                .monitor(job.clone())
                .expect("Initial monitoring failed");

            assert!(
                matches!(status, RuntimeStates::Running),
                "Expected {} to be running before external kills",
                job.id
            );
        }

        // 4. Run external kills, monitoring, and cleanup concurrently.
        let worker_barrier = Arc::new(Barrier::new(JOB_COUNT));

        let worker_handles: Vec<_> = started_jobs
            .into_iter()
            .enumerate()
            .map(|(index, job)| {
                let facade = Arc::clone(&facade);
                let barrier = Arc::clone(&worker_barrier);
                let should_kill = killed_indices.contains(&index);

                thread::spawn(move || {
                    // Synchronize all workers before introducing failures.
                    barrier.wait();

                    if should_kill {
                        // Randomized delay before sending SIGKILL.
                        let delay_ms = ((index * 37 + 19) % 200) as u64;

                        thread::sleep(Duration::from_millis(delay_ms));

                        let process = facade
                            .runtime_state()
                            .get_process(&job.id)
                            .expect("Process state not found");

                        let pid = process.pid.expect("Process PID not found");

                        let result = unsafe {
                            libc::kill(pid as libc::pid_t, libc::SIGKILL)
                        };

                        assert_eq!(
                            result,
                            0,
                            "Failed to SIGKILL process for {}",
                            job.id
                        );

                        println!("{} received external SIGKILL", job.id);

                        // Poll until Monitor detects the signal.
                        let mut detected = false;

                        for attempt in 1..=MAX_MONITOR_ATTEMPTS {
                            let status = facade
                                .monitor(job.clone())
                                .expect("Monitoring killed job failed");

                            if matches!(
                                status,
                                RuntimeStates::Signaled { signal }
                                    if signal == libc::SIGKILL
                            ) {
                                println!(
                                    "{}: SIGKILL detected on attempt {}",
                                    job.id,
                                    attempt
                                );

                                detected = true;
                                break;
                            }

                            thread::sleep(Duration::from_millis(50));
                        }

                        assert!(
                            detected,
                            "{}: Monitor did not detect SIGKILL",
                            job.id
                        );
                    } else {
                        // These jobs should survive the external kill phase.
                        let status = facade
                            .monitor(job.clone())
                            .expect("Monitoring surviving job failed");

                        assert!(
                            matches!(status, RuntimeStates::Running),
                            "{} should still be running",
                            job.id
                        );

                        println!("{} survived external kills", job.id);

                        // Stop surviving jobs through the normal lifecycle.
                        facade
                            .stop(job.clone())
                            .expect("Stopping surviving job failed");
                    }

                    // 5. Every worker cleans up its own job.
                    facade
                        .cleanup(job.clone())
                        .expect("Concurrent job cleanup failed");

                    job
                })
            })
            .collect();

        // 6. Wait for every worker to finish.
        let completed_jobs: Vec<Job> = worker_handles
            .into_iter()
            .map(|handle| handle.join().expect("Failure test worker panicked"))
            .collect();

        // 7. Verify all runtime process and stack state was removed.
        for job in &completed_jobs {
            assert!(
                !facade.runtime_state().contains_process(&job.id),
                "Process state for {} was not removed",
                job.id
            );

            assert!(
                !facade.runtime_state().contains_stack(&job.id),
                "Stack state for {} was not removed",
                job.id
            );
        }

        println!(
            "Completed: {} jobs, {} externally killed, {} normally stopped",
            JOB_COUNT,
            KILL_COUNT,
            JOB_COUNT - KILL_COUNT
        );
    }
}
