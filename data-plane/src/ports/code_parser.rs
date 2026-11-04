use std::io::Read;

pub trait CodeParser {
    fn parse(
        &self,
        source_zip: Box<dyn Read>,
        config_zip: Box<dyn Read>,
    ) -> Result<(), String>;
}