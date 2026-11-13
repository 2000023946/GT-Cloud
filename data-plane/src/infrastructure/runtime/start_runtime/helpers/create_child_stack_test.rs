#[cfg(all(test, target_os = "linux"))]
mod tests {
    use crate::infrastructure::runtime::start_runtime::helpers::create_child_stack::create_child_stack;

    #[test]
    fn creates_stack_with_expected_size() {
        assert_eq!(create_child_stack().len(), 1024 * 1024);
    }

    #[test]
    fn stack_is_zero_initialized() {
        assert!(create_child_stack().iter().all(|byte| *byte == 0));
    }

    #[test]
    fn creates_independent_stacks() {
        let mut first = create_child_stack();
        let second = create_child_stack();

        first[0] = 42;

        assert_eq!(first[0], 42);
        assert_eq!(second[0], 0);
        assert_eq!(first.len(), second.len());
    }
}