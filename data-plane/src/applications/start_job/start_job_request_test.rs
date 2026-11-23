
#[cfg(test)]
mod tests {
    use crate::applications::start_job::start_job_request::StartJobRequest;

use super::*;

    fn valid_request() -> StartJobRequest {
        StartJobRequest {
            id: "job-1".to_string(),
            name: "test-job".to_string(),
            cpu: 1.0,
            memory: 512.0,
            path: "/tmp/test".to_string(),
            file: vec![1, 2, 3],
            command: "python main.py".to_string(),
        }
    }

    #[test]
    fn test_new_success() {
        let request = valid_request();

        let result = StartJobRequest::new(
            request.id,
            request.name,
            request.cpu,
            request.memory,
            request.path,
            request.file,
            request.command,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_empty_id() {
        let mut request = valid_request();
        request.id = "  ".to_string();

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );

        assert!(matches!(
            result,
            Err(error) if error == "Id cannot empty!"
        ));

        
    }

    #[test]
    fn test_new_empty_name() {
        let mut request = valid_request();
        request.name = "".to_string();

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );


        assert!(matches!(
            result,
            Err(error) if error == "Name cannot be empty!"
        ));
    }

    #[test]
    fn test_new_empty_path() {
        let mut request = valid_request();
        request.path = "".to_string();

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );


        assert!(matches!(
            result,
            Err(error) if error == "Code Path cannot be empty!"
        ));

    }

    #[test]
    fn test_new_empty_file() {
        let mut request = valid_request();
        request.file = vec![];

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );

        assert!(matches!(
            result,
            Err(error) if error == "Code File cannot be empty!"
        ));
    }

    #[test]
    fn test_new_zero_cpu() {
        let mut request = valid_request();
        request.cpu = 0.0;

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );


        assert!(matches!(
            result,
            Err(error) if error == "CPU cannot be zero!"
        ));
    }

    #[test]
    fn test_new_zero_memory() {
        let mut request = valid_request();
        request.memory = 0.0;

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );


        assert!(matches!(
            result,
            Err(error) if error == "Memory cannot be zero!"
        ));
    }

    #[test]
    fn test_new_empty_command() {
        let mut request = valid_request();
        request.command = " ".to_string();

        let result = StartJobRequest::new(
            request.id, request.name, request.cpu, request.memory,
            request.path, request.file, request.command,
        );


        assert!(matches!(
            result,
            Err(error) if error == "Start command cannot be empty!"
        ));
    }
}
