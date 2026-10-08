use std::collections::HashMap;

use crate::infrastructure::runtime::states::process_state::ProcessState;

#[derive(Clone)]
pub struct RuntimeState {
    pub processes: HashMap<String, ProcessState>,
}