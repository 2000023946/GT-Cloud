use std::collections::HashMap;

use crate::{
    observability::observability::Observability,
    ports::{logger::Logger, metrics::Metrics},
};

pub struct StopRuntimeTelemetry<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) observability: Observability<L, M>,
}

impl<L, M> StopRuntimeTelemetry<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn new(observability: Observability<L, M>) -> Self {
        Self { observability }
    }

    pub fn log_stop_success(&self, job_id: &str, pid: libc::pid_t) {
        self.observability.logger.info(
            "SIGTERM sent successfully",
            HashMap::from([
                ("job_id".to_string(), job_id.to_string()),
                ("pid".to_string(), pid.to_string()),
            ]),
        );
    }

    pub fn log_stop_failure(&self, job_id: &str, error: &str) {
        self.observability.logger.error(
            "Failed to stop job",
            HashMap::from([
                ("job_id".to_string(), job_id.to_string()),
                ("error".to_string(), error.to_string()),
            ]),
        );
    }

    pub fn record_stop_success(&self) {
        self.observability
            .metrics
            .increment("runtime.stop.successes", 1.0);
    }

    pub fn record_stop_failure(&self) {
        self.observability
            .metrics
            .increment("runtime.stop.failures", 1.0);
    }
}