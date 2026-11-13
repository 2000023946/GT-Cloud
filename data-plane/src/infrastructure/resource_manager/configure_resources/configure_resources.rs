use std::{collections::HashMap, time::Instant};

use crate::{
    domain::job::Job,
    infrastructure::resource_manager::{
        helpers::resource_path_manager::ResourcePathManager,
        states::{
            resource_allocation::ResourceAllocation, resource_manager_state::ResourceManagerState,
        },
    },
    ports::{logger::Logger, metrics::Metrics},
};

const SUCCESS_METRIC: &str = "resource.configure.success";
const FAILURE_METRIC: &str = "resource.configure.failure";
const DURATION_METRIC: &str = "resource.configure.duration_ms";

pub struct ConfigureResources<L, M>
where
    L: Logger,
    M: Metrics,
{
    pub(crate) resource_path_manager: ResourcePathManager,
    pub(crate) logger: L,
    pub(crate) metrics: M,
}

impl<L, M> ConfigureResources<L, M>
where
    L: Logger,
    M: Metrics,
{
    /// Validates and records the resource allocation requested by a job.
    ///
    /// This prepares configuration state only. Applying limits to a process is
    /// deliberately deferred to the future cgroup/runtime integration.
    pub fn configure(&self, state: &mut ResourceManagerState, job: Job) -> Result<(), String> {
        let started_at = Instant::now();
        let job_id = job.id;

        if let Err(message) = validate_job_id(&job_id) {
            return self.configuration_failed(&job_id, message, started_at);
        }

        if job.resources.cpu == 0 {
            return self.configuration_failed(
                &job_id,
                "CPU allocation must be greater than zero",
                started_at,
            );
        }

        if job.resources.memory == 0 {
            return self.configuration_failed(
                &job_id,
                "Memory allocation must be greater than zero",
                started_at,
            );
        }

        let allocation = ResourceAllocation {
            cpu: job.resources.cpu,
            memory: job.resources.memory,
            resource_path: self.resource_path_manager.get_path(&job_id),
        };

        // Insert only after validation so a failed reconfiguration preserves
        // the job's last valid allocation.
        state.allocations.insert(job_id.clone(), allocation);

        let mut fields = HashMap::new();
        fields.insert("job_id".to_string(), job_id);

        self.logger.info("Resources configured for job", fields);
        self.metrics.increment(SUCCESS_METRIC, 1.0);
        self.observe_duration(started_at);

        Ok(())
    }

    fn configuration_failed(
        &self,
        job_id: &str,
        message: &str,
        started_at: Instant,
    ) -> Result<(), String> {
        let mut fields = HashMap::new();
        fields.insert("job_id".to_string(), job_id.to_string());
        fields.insert("reason".to_string(), message.to_string());

        self.logger.error("Failed to configure resources", fields);
        self.metrics.increment(FAILURE_METRIC, 1.0);
        self.observe_duration(started_at);

        Err(message.to_string())
    }

    fn observe_duration(&self, started_at: Instant) {
        self.metrics.observe(
            DURATION_METRIC,
            started_at.elapsed().as_secs_f64() * 1_000.0,
        );
    }
}

fn validate_job_id(job_id: &str) -> Result<(), &'static str> {
    if job_id.trim().is_empty() {
        return Err("Job ID cannot be empty");
    }

    if !job_id
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        return Err("Job ID may contain only letters, numbers, hyphens, and underscores");
    }

    Ok(())
}
