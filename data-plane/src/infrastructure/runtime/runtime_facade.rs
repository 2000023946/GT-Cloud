use crate::{domain::{
    job::Job, runtime_states::runtime_states::RuntimeStates,
    }, infrastructure::runtime::{cleanup::{cleanup_runtime::CleanupRuntime, cleanup_runtime_telemetry::CleanupRuntimeTelemetry}, configure_runtime::{self, configure_runtime::ConfigureRuntime}, helpers::working_directory_manager::{self, WorkingDirectoryManager}, monitor_runtime::monitor_runtime::Monitor, start_runtime::start_runtime::StartRuntime, states::runtime_state::RuntimeState, stop_runtime::{stop_runtime::StopRuntime, stop_runtime_os_adapter::LinuxStopRuntimeOsAdapter, stop_runtime_telemetry::{self, StopRuntimeTelemetry}}}, observability::{self, observability::Observability}, ports::{logger::Logger, metrics::Metrics, runtime::Runtime}
};

pub struct RuntimeFacade<L, M>
where
    L: Logger,
    M: Metrics,
{
    runtime_state: RuntimeState,
    configure_runtime: ConfigureRuntime<L, M>,
    start_runtime: StartRuntime<L, M>,
    stop_runtime: StopRuntime<LinuxStopRuntimeOsAdapter, L, M>,
    monitor: Monitor<L, M>,
    cleanup: CleanupRuntime<L, M>,
}

impl<L, M> RuntimeFacade<L, M>
where
    L: Logger + Clone,
    M: Metrics + Clone,
{
    pub fn new(
        logger: L,
        metrics: M,
    ) -> Self {
        let state = RuntimeState::new();
        let observability = Observability::new(logger.clone(), metrics.clone());
        let working_directory_manager = WorkingDirectoryManager::new();
        let configure_runtime =
            ConfigureRuntime::new(working_directory_manager, observability.clone());

        let start_runtime =
            StartRuntime::new(observability.clone());

        let stop_runtime_os_adapter =
            LinuxStopRuntimeOsAdapter::new();

        let stop_runtime_telemetry = StopRuntimeTelemetry::new(observability.clone());
        let stop_runtime = StopRuntime::new(
            stop_runtime_telemetry,
            stop_runtime_os_adapter,
        );

        
        let monitor =
            Monitor::new(observability.clone());

        let cleanup_telemetry = CleanupRuntimeTelemetry::new(observability.clone());
        let cleanup =
            CleanupRuntime::new(
                cleanup_telemetry,
            );

        Self {
            runtime_state: state,
            configure_runtime,
            start_runtime,
            stop_runtime,
            monitor,
            cleanup,
        }
    }
}

impl<L, M> RuntimeFacade<L, M>
where
    L: Logger + Clone,
    M: Metrics + Clone,
{

    #[cfg(test)]
    pub(crate) fn runtime_state(&self) -> &RuntimeState {
        &self.runtime_state
    }
    pub fn configure(&self, job: Job) -> Result<(), String> {
        return self.configure_runtime.configure(&self.runtime_state, job);
    }

    pub fn start(&self, job: Job) -> Result<(), String> {
        return self.start_runtime.start(&self.runtime_state, job);
    }

    pub fn monitor(&self, job: Job) -> Result<RuntimeStates, String> {
        return self.monitor.monitor(&self.runtime_state, job);
    }

    pub fn stop(&self, job: Job) -> Result<(), String> {
        return self.stop_runtime.stop(&self.runtime_state, job);
    }

    pub fn cleanup(&self, job: Job) -> Result<(), String> {
        return self.cleanup.cleanup(&self.runtime_state, job);
    }
}