pub trait IFileReader {
    fn read(&self, file: String) -> Result<String, String>;
}