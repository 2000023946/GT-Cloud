use std::fs;
pub struct CodeRepository {

}

impl CodeRepository {
    pub fn new() -> Self {
        Self{}
    }

    pub fn create(&self, code: Vec<u8>, path: String) -> Result<(), String> {
        // save code file to disk on path
        let _ = fs::write(path, code);
        Ok(())
    }
}