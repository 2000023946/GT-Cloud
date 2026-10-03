use std::collections::HashMap;

pub trait Logger {
    fn info(
        &self,
        message: &str,
        fields: HashMap<String, String>,
    );

    fn error(
        &self,
        message: &str,
        fields: HashMap<String, String>,
    );
}