
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use crate::infrastructure::runtime::states::process_state::ProcessState;
    use crate::infrastructure::runtime::states::runtime_state::RuntimeState;

    fn make_process() -> ProcessState {
        ProcessState {
            program: "python".to_string(),
            args: vec!["app.py".to_string()],
            working_directory: Some("/jobs/job-1".to_string()),
            environment: vec![("MODE".to_string(), "test".to_string())],
            pid: None,
        }
    }

    fn make_stack() -> Vec<u8> {
        vec![0u8; 1024]
    }

    // Process state tests

    #[test]
    fn creates_empty_state() {
        let state = RuntimeState::new();

        assert_eq!(state.process_count(), 0);
        assert_eq!(state.stack_count(), 0);
    }

    #[test]
    fn adds_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        assert!(state.contains_process("job-1"));
        assert_eq!(state.process_count(), 1);
    }

    #[test]
    fn gets_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        let process = state.get_process("job-1").unwrap();

        assert_eq!(process.program, "python");
        assert_eq!(process.args, vec!["app.py"]);
        assert_eq!(
            process.working_directory,
            Some("/jobs/job-1".to_string())
        );
        assert_eq!(
            process.environment,
            vec![("MODE".to_string(), "test".to_string())]
        );
        assert_eq!(process.pid, None);
    }

    #[test]
    fn get_missing_process_returns_none() {
        let state = RuntimeState::new();

        assert!(state.get_process("missing").is_none());
    }

    #[test]
    fn contains_process_returns_true_for_existing_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        assert!(state.contains_process("job-1"));
    }

    #[test]
    fn contains_process_returns_false_for_missing_process() {
        let state = RuntimeState::new();

        assert!(!state.contains_process("missing"));
    }

    #[test]
    fn process_count_tracks_processes() {
        let state = RuntimeState::new();

        assert_eq!(state.process_count(), 0);

        state.add_process("job-1".to_string(), make_process());
        state.add_process("job-2".to_string(), make_process());

        assert_eq!(state.process_count(), 2);
    }

    #[test]
    fn set_pid_updates_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        assert!(state.set_pid("job-1", 1234).is_ok());
        assert_eq!(state.get_process("job-1").unwrap().pid, Some(1234));
    }

    #[test]
    fn set_pid_missing_process_returns_error() {
        let state = RuntimeState::new();

        let result = state.set_pid("missing", 1234);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing"));
    }

    #[test]
    fn remove_process_returns_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        let removed = state.remove_process("job-1");

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().program, "python");
        assert!(!state.contains_process("job-1"));
        assert_eq!(state.process_count(), 0);
    }

    #[test]
    fn remove_missing_process_returns_none() {
        let state = RuntimeState::new();

        assert!(state.remove_process("missing").is_none());
    }

    #[test]
    fn adding_same_job_replaces_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        let replacement = ProcessState {
            program: "node".to_string(),
            args: vec!["server.js".to_string()],
            working_directory: None,
            environment: vec![],
            pid: None,
        };

        state.add_process("job-1".to_string(), replacement);

        assert_eq!(state.process_count(), 1);
        assert_eq!(state.get_process("job-1").unwrap().program, "node");
    }

    #[test]
    fn multiple_jobs_are_stored_independently() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());

        let second = ProcessState {
            program: "node".to_string(),
            args: vec![],
            working_directory: None,
            environment: vec![],
            pid: None,
        };

        state.add_process("job-2".to_string(), second);

        assert_eq!(state.get_process("job-1").unwrap().program, "python");
        assert_eq!(state.get_process("job-2").unwrap().program, "node");
        assert_eq!(state.process_count(), 2);
    }

    #[test]
    fn concurrent_writes_are_safe() {
        let state = Arc::new(RuntimeState::new());
        let mut handles = vec![];

        for i in 0..20 {
            let state = Arc::clone(&state);

            handles.push(thread::spawn(move || {
                state.add_process(
                    format!("job-{}", i),
                    make_process(),
                );
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(state.process_count(), 20);
    }

    #[test]
    fn concurrent_reads_are_safe() {
        let state = Arc::new(RuntimeState::new());

        state.add_process("job-1".to_string(), make_process());

        let mut handles = vec![];

        for _ in 0..20 {
            let state = Arc::clone(&state);

            handles.push(thread::spawn(move || {
                let process = state.get_process("job-1").unwrap();
                assert_eq!(process.program, "python");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn concurrent_reads_and_writes_are_safe() {
        let state = Arc::new(RuntimeState::new());

        state.add_process("job-1".to_string(), make_process());

        let reader_state = Arc::clone(&state);

        let reader = thread::spawn(move || {
            for _ in 0..100 {
                let _ = reader_state.get_process("job-1");
            }
        });

        let writer_state = Arc::clone(&state);

        let writer = thread::spawn(move || {
            for pid in 1000..1100 {
                writer_state.set_pid("job-1", pid).unwrap();
            }
        });

        reader.join().unwrap();
        writer.join().unwrap();

        assert!(state.get_process("job-1").unwrap().pid.is_some());
    }

    // Stack tests

    #[test]
    fn adds_stack() {
        let state = RuntimeState::new();

        state.add_stack("job-1".to_string(), make_stack());

        assert!(state.contains_stack("job-1"));
        assert_eq!(state.stack_count(), 1);
    }

    #[test]
    fn contains_stack_returns_true_for_existing_stack() {
        let state = RuntimeState::new();

        state.add_stack("job-1".to_string(), make_stack());

        assert!(state.contains_stack("job-1"));
    }

    #[test]
    fn contains_stack_returns_false_for_missing_stack() {
        let state = RuntimeState::new();

        assert!(!state.contains_stack("missing"));
    }

    #[test]
    fn removes_stack() {
        let state = RuntimeState::new();

        state.add_stack("job-1".to_string(), make_stack());

        let removed = state.remove_stack("job-1");

        assert!(removed.is_some());
        assert_eq!(removed.unwrap().len(), 1024);
        assert!(!state.contains_stack("job-1"));
        assert_eq!(state.stack_count(), 0);
    }

    #[test]
    fn remove_missing_stack_returns_none() {
        let state = RuntimeState::new();

        assert!(state.remove_stack("missing").is_none());
    }

    #[test]
    fn adding_same_job_replaces_stack() {
        let state = RuntimeState::new();

        state.add_stack("job-1".to_string(), vec![1, 2, 3]);
        state.add_stack("job-1".to_string(), vec![4, 5]);

        assert_eq!(state.stack_count(), 1);

        let stack = state.with_stack("job-1", |stack| stack.clone()).unwrap();

        assert_eq!(stack, vec![4, 5]);
    }

    #[test]
    fn multiple_stacks_are_stored_independently() {
        let state = RuntimeState::new();

        state.add_stack("job-1".to_string(), vec![1, 2]);
        state.add_stack("job-2".to_string(), vec![3, 4, 5]);

        assert_eq!(state.stack_count(), 2);

        let first = state.with_stack("job-1", |stack| stack.clone()).unwrap();
        let second = state.with_stack("job-2", |stack| stack.clone()).unwrap();

        assert_eq!(first, vec![1, 2]);
        assert_eq!(second, vec![3, 4, 5]);
    }

    #[test]
    fn with_stack_can_mutate_stack() {
        let state = RuntimeState::new();

        state.add_stack("job-1".to_string(), vec![1, 2, 3]);

        state.with_stack("job-1", |stack| {
            stack.push(4);
            stack[0] = 9;
        }).unwrap();

        let stack = state.with_stack("job-1", |stack| stack.clone()).unwrap();

        assert_eq!(stack, vec![9, 2, 3, 4]);
    }

    #[test]
    fn with_stack_missing_stack_returns_error() {
        let state = RuntimeState::new();

        let result = state.with_stack("missing", |stack| stack.len());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing"));
    }

    #[test]
    fn concurrent_stack_writes_are_safe() {
        let state = Arc::new(RuntimeState::new());
        let mut handles = vec![];

        for i in 0..20 {
            let state = Arc::clone(&state);

            handles.push(thread::spawn(move || {
                state.add_stack(
                    format!("job-{}", i),
                    make_stack(),
                );
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(state.stack_count(), 20);
    }

    #[test]
    fn concurrent_stack_reads_are_safe() {
        let state = Arc::new(RuntimeState::new());

        state.add_stack("job-1".to_string(), make_stack());

        let mut handles = vec![];

        for _ in 0..20 {
            let state = Arc::clone(&state);

            handles.push(thread::spawn(move || {
                let length = state
                    .with_stack("job-1", |stack| stack.len())
                    .unwrap();

                assert_eq!(length, 1024);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn concurrent_stack_reads_and_writes_are_safe() {
        let state = Arc::new(RuntimeState::new());

        state.add_stack("job-1".to_string(), make_stack());

        let reader_state = Arc::clone(&state);

        let reader = thread::spawn(move || {
            for _ in 0..100 {
                let _ = reader_state
                    .with_stack("job-1", |stack| stack.len())
                    .unwrap();
            }
        });

        let writer_state = Arc::clone(&state);

        let writer = thread::spawn(move || {
            for _ in 0..100 {
                writer_state
                    .with_stack("job-1", |stack| {
                        stack[0] = stack[0].wrapping_add(1);
                    })
                    .unwrap();
            }
        });

        reader.join().unwrap();
        writer.join().unwrap();

        assert_eq!(state.stack_count(), 1);
    }

    // Process and stack independence tests

    #[test]
    fn stacks_and_processes_are_independent() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());
        state.add_stack("job-1".to_string(), make_stack());

        assert!(state.contains_process("job-1"));
        assert!(state.contains_stack("job-1"));

        assert_eq!(state.process_count(), 1);
        assert_eq!(state.stack_count(), 1);
    }

    #[test]
    fn removing_process_does_not_remove_stack() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());
        state.add_stack("job-1".to_string(), make_stack());

        state.remove_process("job-1");

        assert!(!state.contains_process("job-1"));
        assert!(state.contains_stack("job-1"));
    }

    #[test]
    fn removing_stack_does_not_remove_process() {
        let state = RuntimeState::new();

        state.add_process("job-1".to_string(), make_process());
        state.add_stack("job-1".to_string(), make_stack());

        state.remove_stack("job-1");

        assert!(state.contains_process("job-1"));
        assert!(!state.contains_stack("job-1"));
    }
}

