use std::collections::HashMap;

#[derive(Clone)]
pub struct RuntimeState {
    pub processes: HashMap<String, ProcessState>,
}