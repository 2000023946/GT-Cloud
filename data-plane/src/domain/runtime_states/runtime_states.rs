#[derive(Clone)]
pub enum RuntimeStates {
    Running,
    Exited { code: i32 },
    Signaled { signal: i32 },
    Error,
    NotFound,
}