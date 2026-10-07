pub struct MonitorJobsService<R, RM, N, S> {
    runtime: R,
    resources: RM,
    network: N,
    state_repository: S,
}