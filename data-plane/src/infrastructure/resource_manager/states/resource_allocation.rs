/// The resource limits prepared for one job.
///
/// This is configuration state, not live utilization. Runtime usage is
/// reported separately through the domain `ResourceState` type.
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceAllocation {
    pub cpu: f32,
    pub memory: f32,
    pub resource_path: String,
}
