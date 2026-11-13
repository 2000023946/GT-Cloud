#[derive(Clone)]
pub enum JobStatus {
    Received,
    Starting,
    Running,
    Completed,
    Failed,
    Stopped,
}