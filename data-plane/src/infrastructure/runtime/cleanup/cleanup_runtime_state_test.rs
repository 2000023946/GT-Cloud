
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::infrastructure::runtime::cleanup::cleanup_runtime_state::CleanupRuntimeState;
    use crate::infrastructure::runtime::states::{
        process_state::ProcessState,
        runtime_state::RuntimeState,
    };

    fn create_process() -> ProcessState {
        ProcessState {
            program: "sleep".to_string(),
            args: vec!["30".to_string()],
            working_directory: Some("/tmp".to_string()),
            environment: vec![("TEST".to_string(), "true".to_string())],
            pid: Some(1234),
        }
    }

    // RuntimeState trait implementation tests

    #[test]
    fn removes_existing_process() {
        let state = RuntimeState::new();
        state.add_process("job-1".to_string(), create_process());

        let removed =
            <RuntimeState as CleanupRuntimeState>::remove_process(&state, "job-1")
                .unwrap();

        assert_eq!(removed.unwrap().program, "sleep");
        assert!(!state.contains_process("job-1"));
    }

    #[test]
    fn returns_none_when_process_does_not_exist() {
        let state = RuntimeState::new();

        let removed =
            <RuntimeState as CleanupRuntimeState>::remove_process(&state, "missing-job")
                .unwrap();

        assert!(removed.is_none());
    }

    #[test]
    fn removes_existing_stack() {
        let state = RuntimeState::new();
        let stack = vec![1, 2, 3, 4];

        state.add_stack("job-1".to_string(), stack.clone());

        let removed =
            <RuntimeState as CleanupRuntimeState>::remove_stack(&state, "job-1")
                .unwrap();

        assert_eq!(removed, Some(stack));
        assert!(!state.contains_stack("job-1"));
    }

    #[test]
    fn returns_none_when_stack_does_not_exist() {
        let state = RuntimeState::new();

        let removed =
            <RuntimeState as CleanupRuntimeState>::remove_stack(&state, "missing-job")
                .unwrap();

        assert!(removed.is_none());
    }

    // Arc forwarding implementation tests

    #[test]
    fn arc_forwards_remove_process() {
        let state = Arc::new(RuntimeState::new());
        state.add_process("job-1".to_string(), create_process());

        let removed = CleanupRuntimeState::remove_process(&state, "job-1")
            .unwrap()
            .unwrap();

        assert_eq!(removed.program, "sleep");
        assert!(!state.contains_process("job-1"));
    }

    #[test]
    fn arc_forwards_remove_stack() {
        let state = Arc::new(RuntimeState::new());
        let stack = vec![5, 6, 7, 8];

        state.add_stack("job-1".to_string(), stack.clone());

        let removed = CleanupRuntimeState::remove_stack(&state, "job-1")
            .unwrap();

        assert_eq!(removed, Some(stack));
        assert!(!state.contains_stack("job-1"));
    }

    #[test]
    fn arc_returns_none_for_missing_process() {
        let state = Arc::new(RuntimeState::new());

        let removed = CleanupRuntimeState::remove_process(&state, "missing-job")
            .unwrap();

        assert!(removed.is_none());
    }

    #[test]
    fn arc_returns_none_for_missing_stack() {
        let state = Arc::new(RuntimeState::new());

        let removed = CleanupRuntimeState::remove_stack(&state, "missing-job")
            .unwrap();

        assert!(removed.is_none());
    }
}
