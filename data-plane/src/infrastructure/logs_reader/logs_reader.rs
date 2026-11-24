use crate::ports::{i_file_reader::IFileReader, i_logs_reader::ILogsReader};

pub struct LogsReader<R: IFileReader> {
    pub file_reader: R,
}


impl<R: IFileReader> ILogsReader for LogsReader<R> {
    
    fn read_stdout(&self, path: String) -> Result<String, String> {
        match self.file_reader.read(path + "/stdout.txt") {
            Ok(content) => Ok(content),
            Err(error) => Err(error.to_string()),
        }
    }

    fn read_stderr(&self, path: String) -> Result<String, String> {
        match self.file_reader.read(path + "/stderr.txt") {
            Ok(content) => Ok(content),
            Err(error) => Err(error.to_string()),
        }
    }
}