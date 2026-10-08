/// The resource limits prepared for one job.
///
/// This is configuration state, not live utilization. Runtime usage is
/// reported separately through the domain `ResourceState` type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceAllocation {
    pub cpu: u32,
    pub memory: u64,
    pub resource_path: String,
}
