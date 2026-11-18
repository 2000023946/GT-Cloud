
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Barrier};
    use std::thread;

    use crate::domain::{
        app::App,
        job::{Job, JobStatus},
        resource::Resource,
    };
    use crate::domain::runtime_states::runtime_states::RuntimeStates;
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
        let job_id = format!("concurrent-job-{id}");

        Job {
            id: job_id.clone(),
            name: format!("Concurrent Test Job {id}"),
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

    #[test]
    fn test_multiple_jobs_run_concurrently() {
        const JOB_COUNT: usize = 4;

        let facade = Arc::new(RuntimeFacade::new(TestLogger, TestMetrics));
        let start_barrier = Arc::new(Barrier::new(JOB_COUNT));

        let jobs: Vec<Job> = (0..JOB_COUNT).map(create_test_job).collect();

        // Configure each job on a separate thread, then synchronize starts.
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

        // Verify that every job is independently running.
        for job in &started_jobs {
            let status = facade
                .monitor(job.clone())
                .expect("Monitoring concurrent job failed");

            assert!(
                matches!(status, RuntimeStates::Running),
                "Expected job {} to be running",
                job.id
            );
        }

        // Stop all jobs concurrently.
        let stop_barrier = Arc::new(Barrier::new(JOB_COUNT));

        let stop_handles: Vec<_> = started_jobs
            .iter()
            .cloned()
            .map(|job| {
                let facade = Arc::clone(&facade);
                let barrier = Arc::clone(&stop_barrier);

                thread::spawn(move || {
                    barrier.wait();

                    facade
                        .stop(job.clone())
                        .expect("Concurrent job stop failed");

                    job
                })
            })
            .collect();

        let stopped_jobs: Vec<Job> = stop_handles
            .into_iter()
            .map(|handle| handle.join().expect("Job stop thread panicked"))
            .collect();

        // Clean up each job concurrently after all stop operations finish.
        let cleanup_handles: Vec<_> = stopped_jobs
            .into_iter()
            .map(|job| {
                let facade = Arc::clone(&facade);

                thread::spawn(move || {
                    facade
                        .cleanup(job.clone())
                        .expect("Concurrent job cleanup failed");
                })
            })
            .collect();

        for handle in cleanup_handles {
            handle
                .join()
                .expect("Job cleanup thread panicked");
        }
    }
}
