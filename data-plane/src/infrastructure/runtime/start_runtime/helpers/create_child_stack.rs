#[cfg(target_os = "linux")]
pub fn create_child_stack() -> Vec<u8> {
    vec![0u8; 1024 * 1024]
}