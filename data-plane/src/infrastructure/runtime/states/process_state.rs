#[derive(Clone)]
pub struct ProcessState {
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Vec<(String, String)>,
    pub pid: Option<u32>,
}