
#[cfg(test)]
mod tests {
    use crate::domain::resource::Resource;
use crate::domain::globals::globals::MAX_CPU;
use crate::domain::globals::globals::MAX_MEMORY;
use super::*;

    #[test]
    fn test_new_success() {
        let result = Resource::new(1.0, 512.0);

        assert!(result.is_ok());

        let resource = result.unwrap();
        assert_eq!(resource.cpu, 1.0);
        assert_eq!(resource.memory, 512.0);
    }

    #[test]
    fn test_new_zero_cpu() {
        let result = Resource::new(0.0, 512.0);

        assert!(matches!(
            result,
            Err(error) if error == "CPU cannot be negative!"
        ));
    }

    #[test]
    fn test_new_negative_cpu() {
        let result = Resource::new(-1.0, 512.0);

        assert!(matches!(
            result,
            Err(error) if error == "CPU cannot be negative!"
        ));
    }

    #[test]
    fn test_new_cpu_too_big() {
        let result = Resource::new(MAX_CPU as f32, 512.0);

        assert!(matches!(
            result,
            Err(error) if error == "CPU too big!"
        ));
    }

    #[test]
    fn test_new_zero_memory() {
        let result = Resource::new(1.0, 0.0);

        assert!(matches!(
            result,
            Err(error) if error == "Memory cannot be negative!"
        ));
    }

    #[test]
    fn test_new_negative_memory() {
        let result = Resource::new(1.0, -512.0);

        assert!(matches!(
            result,
            Err(error) if error == "Memory cannot be negative!"
        ));
    }

    #[test]
    fn test_new_memory_too_big() {
        let result = Resource::new(1.0, MAX_MEMORY as f32);

        assert!(matches!(
            result,
            Err(error) if error == "Memory too big!"
        ));
    }
}
