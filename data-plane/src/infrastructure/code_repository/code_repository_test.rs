#[cfg(test)]
mod tests {
    use crate::infrastructure::code_repository::code_repository::CodeRepository;

use super::*;
    use std::fs;

    #[test]
    fn test_create_file() {
        let repository = CodeRepository::new();
        let path = "test_create_file.bin".to_string();
        let code: Vec<u8> = vec![10, 20, 30, 40, 50];

        let result = repository.create(code.clone(), path.clone());

        assert!(result.is_ok());
        assert!(std::path::Path::new(&path).exists());

        let saved_code = fs::read(&path).expect("Failed to read created file");

        assert_eq!(saved_code, code);

        fs::remove_file(&path).expect("Failed to delete test file");
        assert!(!std::path::Path::new(&path).exists());
    }

    #[test]
    fn test_create_empty_file() {
        let repository = CodeRepository::new();
        let path = "test_create_empty_file.bin".to_string();
        let code: Vec<u8> = vec![];

        let result = repository.create(code, path.clone());

        assert!(result.is_ok());
        assert!(std::path::Path::new(&path).exists());

        let saved_code = fs::read(&path).expect("Failed to read created file");

        assert!(saved_code.is_empty());

        fs::remove_file(&path).expect("Failed to delete test file");
    }
}