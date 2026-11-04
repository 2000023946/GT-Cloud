
#[cfg(all(test, target_os = "linux"))]
use super::create_process::create_process_with;

#[cfg(all(test, target_os = "linux"))]
use libc::{
    CLONE_NEWIPC, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUSER, CLONE_NEWUTS, SIGCHLD,
};

#[cfg(all(test, target_os = "linux"))]
use std::ffi::c_void;

#[cfg(all(test, target_os = "linux"))]
#[test]
fn create_process_with_returns_pid_on_success() {
    let mut stack = vec![0_u8; 1024];
    let mut called = false;

    let result = create_process_with(
        &mut stack,
        (),
        |stack_top, flags, context_ptr| {
            called = true;

            assert!(!stack_top.is_null());
            assert_eq!(
                flags,
                CLONE_NEWPID
                    | CLONE_NEWNS
                    | CLONE_NEWUTS
                    | CLONE_NEWIPC
                    | CLONE_NEWUSER
                    | SIGCHLD
            );
            assert!(!context_ptr.is_null());

            // The mock does not create a child to own the context.
            // Reclaim it to avoid leaking memory in this test.
            unsafe {
                drop(Box::from_raw(context_ptr as *mut ()));
            }

            123 as libc::pid_t
        },
        || panic!("failure callback should not run on success"),
    );

    assert!(called);
    assert_eq!(result.unwrap(), 123);
}

#[cfg(all(test, target_os = "linux"))]
#[test]
fn create_process_with_returns_error_on_clone_failure() {
    let mut stack = vec![0_u8; 1024];
    let mut failure_called = false;

    let result = create_process_with(
        &mut stack,
        (),
        |stack_top, _flags, context_ptr| {
            assert!(!stack_top.is_null());
            assert!(!context_ptr.is_null());

            -1 as libc::pid_t
        },
        || {
            failure_called = true;
        },
    );

    assert!(failure_called);
    assert!(result.is_err());
}