use std::collections::HashMap;

use crate::infrastructure::resource_manager::states::resource_allocation::ResourceAllocation;

/// In-memory registry of the resource allocations prepared on this worker.
#[derive(Clone, Debug, Default)]
pub struct ResourceManagerState {
    pub allocations: HashMap<String, ResourceAllocation>,
}
