
#[cfg(test)]
mod tests {
    use crate::infrastructure::file_reader::file_reader::FileReader;
use crate::ports::i_file_reader::IFileReader;

use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_dir() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir()
            .join(format!("file-reader-test-{}", timestamp));

        fs::create_dir_all(&path).unwrap();

        path.to_string_lossy().to_string()
    }

    #[test]
    fn test_read_success() {
        let directory = create_temp_dir();
        let file_path = format!("{}/test.txt", directory);

        fs::write(&file_path, "Hello, GT Cloud!").unwrap();

        let reader = FileReader {};
        let result = reader.read(file_path.clone());

        assert_eq!(result.unwrap(), "Hello, GT Cloud!");

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn test_read_empty_file() {
        let directory = create_temp_dir();
        let file_path = format!("{}/empty.txt", directory);

        fs::write(&file_path, "").unwrap();

        let reader = FileReader {};
        let result = reader.read(file_path);

        assert_eq!(result.unwrap(), "");

        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn test_read_file_not_found() {
        let directory = create_temp_dir();
        let file_path = format!("{}/missing.txt", directory);

        let reader = FileReader {};
        let result = reader.read(file_path);

        assert!(result.is_err());


    }

    #[test]
    fn test_read_invalid_path() {
        let reader = FileReader {};

        let result = reader.read("\0invalid_path".to_string());

        assert!(result.is_err());
    }
}
