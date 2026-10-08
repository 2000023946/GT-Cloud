# Resource `configure()`

## Purpose

`configure()` validates a job's requested CPU and memory and records the
allocation that the worker will apply when the job is executed. It prepares
resource-manager state; it does not enforce operating-system limits yet.

The operation follows this sequence:

```text
Job
  -> validate job ID and requested resources
  -> generate the job's resource path
  -> create ResourceAllocation
  -> store it in ResourceManagerState
  -> record logs and metrics
```

## Inputs and assumptions

The input is the domain `Job`. This operation uses:

* `Job.id` as the allocation key and as part of the resource path.
* `Job.resources.cpu` as the requested CPU allocation.
* `Job.resources.memory` as the requested memory allocation.

The current domain model does not define CPU or memory units. This operation
therefore preserves both values without converting them. The units must be
agreed upon before Linux resource enforcement is implemented. Both values must
be greater than zero.

Job IDs may contain ASCII letters, numbers, hyphens, and underscores. This
restriction prevents an ID from escaping the configured resource root when it
is used in a path.

## State model

`ResourceAllocation` is the prepared configuration for one job:

* Requested CPU
* Requested memory
* Generated resource path

`ResourceManagerState` is an in-memory map from job ID to
`ResourceAllocation`. It is resource-manager bookkeeping, not persisted state
and not live resource usage.

The existing domain `ResourceState` has a different purpose: a future
`monitor()` implementation will return it to describe observed CPU and memory
usage.

## Reconfiguration contract

Configuring an existing job with another valid request replaces its stored
allocation. Invalid reconfiguration leaves the previous valid allocation
unchanged because all validation occurs before state is modified.

## Failure contract

Configuration fails when:

* The job ID is empty or contains unsupported characters.
* CPU is zero.
* Memory is zero.

On failure, the operation returns an error, does not add or replace allocation
state, emits an error log, increments the failure counter, and records the
operation duration.

## Observability

Successful configuration emits an informational log containing `job_id` and
increments:

```text
resource.configure.success
```

Failed configuration emits an error log containing `job_id` and `reason` and
increments:

```text
resource.configure.failure
```

Every attempt records milliseconds under:

```text
resource.configure.duration_ms
```

## Explicitly out of scope

This operation does not:

* Create or modify Linux cgroups.
* Attach a process to a resource allocation.
* Enforce CPU or memory limits.
* Monitor resource usage.
* Clean up resource controls.
* Persist allocations across data-plane restarts.

Before cgroup enforcement is added, the Resource and Runtime owners must define
how a process starts inside its allocation. The resource units and supported
Linux/cgroup version must also be documented.

## Testing

Functional tests cover valid allocation, multiple jobs, valid
reconfiguration, path generation, and stored values. Failure tests cover empty,
whitespace, and unsafe job IDs, zero resources, state preservation, and
success/failure observability.

The performance test configures 1,000, 10,000, and 100,000 in-memory
allocations and reports total time, average time per job, and throughput. It
measures validation, path generation, map insertion, and no-op observability;
it is not a cgroup performance benchmark.

An unoptimized local test run produced:

| Jobs | Total time | Average per job | Throughput |
| ---: | ---: | ---: | ---: |
| 1,000 | 1.56 ms | 1.56 us | 640,239 jobs/sec |
| 10,000 | 14.84 ms | 1.48 us | 673,981 jobs/sec |
| 100,000 | 150.63 ms | 1.51 us | 663,897 jobs/sec |

These results are illustrative and depend on hardware, build mode, and system
load. They should be regenerated when the implementation or benchmark
environment changes.
