#[derive(Clone)]
pub struct App {
    pub id: String,
    pub code_path: String,
    pub command: String,
    pub environment: Vec<(String, String)>,
}
