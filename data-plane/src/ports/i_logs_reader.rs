pub trait ILogsReader  {
    fn read_stdout(&self, path: String) -> Result<String, String>;
    fn read_stderr(&self, path: String) -> Result<String, String>;
}