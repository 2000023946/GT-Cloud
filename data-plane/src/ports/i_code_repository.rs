pub trait ICodeRepository {
    fn create(&self, code: Vec<u8>, path: String) -> Result<(), String>;
}
