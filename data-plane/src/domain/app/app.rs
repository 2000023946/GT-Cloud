#[derive(Clone)]
pub struct App {
    pub id: String,
    pub code_path: String,
    pub command: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_app_with_expected_fields() {
        let app = App {
            id: "app-123".to_string(),
            code_path: "/home/user/app".to_string(),
            command: "python main.py".to_string(),
        };

        assert_eq!(app.id, "app-123");
        assert_eq!(app.code_path, "/home/user/app");
        assert_eq!(app.command, "python main.py");
    }

    #[test]
    fn clones_app_with_same_fields() {
        let app = App {
            id: "app-123".to_string(),
            code_path: "/home/user/app".to_string(),
            command: "python main.py".to_string(),
        };

        let cloned_app = app.clone();

        assert_eq!(cloned_app.id, app.id);
        assert_eq!(cloned_app.code_path, app.code_path);
        assert_eq!(cloned_app.command, app.command);
    }
}