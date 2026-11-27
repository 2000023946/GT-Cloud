use std::fs;

use crate::ports::i_file_reader::IFileReader;

pub struct FileReader {}

impl IFileReader for FileReader {
    fn read(&self, file: String) -> Result<String, String> {
        return fs::read_to_string(file).map_err(|err| err.to_string())
    }
}