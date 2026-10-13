
use crate::infrastructure::runtime::states::process_state::ProcessState;
use crate::infrastructure::runtime::states::runtime_state::RuntimeState;

use std::hint::black_box;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
enum Workload {
    Insert,
    Get,
    Contains,
    UpdatePid,
    Remove,
    Mixed,
}

impl Workload {
    fn name(self) -> &'static str {
        match self {
            Self::Insert => "Concurrent insert",
            Self::Get => "Concurrent get",
            Self::Contains => "Concurrent contains",
            Self::UpdatePid => "Concurrent PID update",
            Self::Remove => "Concurrent remove",
            Self::Mixed => "Concurrent mixed (80/10/10)",
        }
    }
}

fn make_process(_job_id: usize) -> ProcessState {
    ProcessState {
        program: "test-program".to_string(),
        args: vec!["--benchmark".to_string()],
        working_directory: Some("/tmp".to_string()),
        environment: vec![("MODE".to_string(), "benchmark".to_string())],
        pid: None,
    }
}

fn report(label: &str, operations: usize, elapsed: Duration) {
    let seconds = elapsed.as_secs_f64();

    println!(
        "{:<27} | ops: {:>7} | time: {:>9.3} ms | throughput: {:>12.0} ops/sec | avg: {:>9.1} ns/op",
        label,
        operations,
        seconds * 1000.0,
        operations as f64 / seconds,
        seconds * 1_000_000_000.0 / operations as f64,
    );
}

fn populate_state(n: usize) -> RuntimeState {
    let state = RuntimeState::new();

    for i in 0..n {
        state.add_process(format!("job-{i}"), make_process(i));
    }

    state
}

fn run_concurrent(
    state: &RuntimeState,
    n: usize,
    thread_count: usize,
    workload: Workload,
) -> Duration {
    let start = Instant::now();

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(thread_count);

        for thread_id in 0..thread_count {
            let start_index = thread_id * n / thread_count;
            let end_index = (thread_id + 1) * n / thread_count;

            let handle = scope.spawn(move || {
                for i in start_index..end_index {
                    let job_id = format!("job-{i}");

                    match workload {
                        Workload::Insert => {
                            state.add_process(
                                format!("insert-{i}"),
                                make_process(i),
                            );
                        }

                        Workload::Get => {
                            black_box(state.get_process(&job_id));
                        }

                        Workload::Contains => {
                            black_box(state.contains_process(&job_id));
                        }

                        Workload::UpdatePid => {
                            state
                                .set_pid(&job_id, i as u32 + 1)
                                .expect("job should exist");
                        }

                        Workload::Remove => {
                            black_box(
                                state.remove_process(&format!("insert-{i}")),
                            );
                        }

                        Workload::Mixed => {
                            match i % 10 {
                                0..=7 => {
                                    black_box(state.get_process(&job_id));
                                }
                                8 => {
                                    black_box(state.contains_process(&job_id));
                                }
                                _ => {
                                    state
                                        .set_pid(&job_id, i as u32 + 1)
                                        .expect("job should exist");
                                }
                            }
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("benchmark worker panicked");
        }
    });

    start.elapsed()
}

fn benchmark_concurrent_size(n: usize) {
    println!("\n========== Concurrent RuntimeState: {n} jobs ==========");

    let state = populate_state(n);

    for thread_count in [1, 2, 4, 8, 16] {
        println!("\n--- Threads: {thread_count} ---");

        // Insert uses unique IDs. Remove immediately afterward to clean up.
        let insert_time =
            run_concurrent(&state, n, thread_count, Workload::Insert);
        report(
            Workload::Insert.name(),
            n,
            insert_time,
        );

        let get_time =
            run_concurrent(&state, n, thread_count, Workload::Get);
        report(Workload::Get.name(), n, get_time);

        let contains_time =
            run_concurrent(&state, n, thread_count, Workload::Contains);
        report(Workload::Contains.name(), n, contains_time);

        let update_time =
            run_concurrent(&state, n, thread_count, Workload::UpdatePid);
        report(Workload::UpdatePid.name(), n, update_time);

        let remove_time =
            run_concurrent(&state, n, thread_count, Workload::Remove);
        report(Workload::Remove.name(), n, remove_time);

        let mixed_time =
            run_concurrent(&state, n, thread_count, Workload::Mixed);
        report(Workload::Mixed.name(), n, mixed_time);

        assert_eq!(state.process_count(), n);
    }
}

#[test]
fn runtime_state_concurrent_performance_1k_10k_100k() {
    println!("\nRuntimeState concurrent performance benchmark");
    println!("Build profile: release");
    println!("Each workload performs n total operations across all threads.");
    println!("Mixed workload: 80% get, 10% contains, 10% PID update.");

    for n in [1_000, 10_000, 100_000] {
        benchmark_concurrent_size(n);
    }
}

#[test]
fn runtime_state_sequential_performance_1k_10k_100k() {
    println!("\nRuntimeState sequential performance benchmark");

    for n in [1_000, 10_000, 100_000] {
        println!("\n========== Sequential RuntimeState: {n} jobs ==========");

        let state = RuntimeState::new();

        let start = Instant::now();
        for i in 0..n {
            state.add_process(format!("job-{i}"), make_process(i));
        }
        report("Insert processes", n, start.elapsed());

        let start = Instant::now();
        for i in 0..n {
            black_box(state.get_process(&format!("job-{i}")));
        }
        report("Get processes", n, start.elapsed());

        let start = Instant::now();
        for i in 0..n {
            black_box(state.contains_process(&format!("job-{i}")));
        }
        report("Contains processes", n, start.elapsed());

        let start = Instant::now();
        for i in 0..n {
            state
                .set_pid(&format!("job-{i}"), i as u32 + 1)
                .expect("job should exist");
        }
        report("Update process PID", n, start.elapsed());

        let start = Instant::now();
        for i in 0..n {
            black_box(state.remove_process(&format!("job-{i}")));
        }
        report("Remove processes", n, start.elapsed());

        assert_eq!(state.process_count(), 0);
    }
}

