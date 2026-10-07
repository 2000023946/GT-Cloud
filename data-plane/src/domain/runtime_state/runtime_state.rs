#[derive(Clone)]
pub enum RuntimeState {
    Running,
    Exited { code: i32 },
    Signaled { signal: i32 },
    NotFound,
}