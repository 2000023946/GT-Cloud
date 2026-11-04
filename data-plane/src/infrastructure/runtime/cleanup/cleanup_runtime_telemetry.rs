use std::collections::HashMap;

use crate::{
    observability::observability::Observability,
    ports::{logger::Logger, metrics::Metrics},
};

pub struct CleanupRuntimeTelemetry<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) observability: Observability<L, M>,
}

impl<L, M> CleanupRuntimeTelemetry<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub fn new(observability: Observability<L, M>) -> Self {
        Self { observability }
    }

    pub fn log_cleanup(&self, job_id: &str) {
        let mut fields = HashMap::new();
        fields.insert("job_id".to_string(), job_id.to_string());

        self.observability.logger.info(
            "Successfully cleaned up runtime",
            fields,
        );

        self.observability
            .metrics
            .increment("runtime_cleanup_success", 1.0);
    }

    pub fn log_failure(&self, job_id: &str, error: &str) {
        let mut fields = HashMap::new();
        fields.insert("job_id".to_string(), job_id.to_string());
        fields.insert("error".to_string(), error.to_string());

        self.observability.logger.error(
            "Failed to clean up runtime",
            fields,
        );

        self.observability
            .metrics
            .increment("runtime_cleanup_failure", 1.0);
    }
}