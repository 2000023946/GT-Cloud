use std::collections::HashMap;

use crate::{
    observability::observability::Observability,
    ports::{
        logger::Logger,
        metrics::Metrics,
    },
};

pub fn record_success<L, M>(
    observability: &Observability<L, M>,
    job_id: &str,
    pid: u32,
)
where
    L: Logger,
    M: Metrics,
{
    let mut fields =
        HashMap::new();

    fields.insert(
        "job_id".to_string(),
        job_id.to_string(),
    );

    fields.insert(
        "pid".to_string(),
        pid.to_string(),
    );

    observability
        .logger
        .info(
            "Runtime started job",
            fields,
        );

    observability
        .metrics
        .increment(
            "runtime.start.success",
            1.0,
        );
}