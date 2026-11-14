# Data Plane API Specification

**Status:** Draft v0.1, for review  

---

## 1. Purpose and Scope

This document specifies the API of the **Data Plane**: the part of the platform that runs jobs on a worker and manages their execution, resources and networking. It defines every operation the Data Plane exposes, its inputs, outputs, validation rules, errors, and failure behavior.

**In scope**

- Worker API operations: `startJob`, `stopJob`, `updateJob`, `getJob`, `listJobs`, plus supporting operations (`deleteJob`, `getHealth`, and others in section 7).
- Resource queries: `getResources` (worker capacity and availability) and `getJobResources` (per-job usage).
- Request/response formats, validation, error handling, failure behavior.
- Responsibilities of each operation and assumptions requiring review.

**Out of scope**

- The Control Plane API and the Control Plane to Data Plane communication protocol (HTTP, gRPC, message queue, and so on).
- Implementing the API methods (as this is in review stage).

---

## 2. Where the API Sits

```text
Control Plane
      |
 Worker Client
      |
 Data Plane API        <- this spec
      |
 Applications          <- one use case per API operation
      |
 Execution
   +--+------------+
   |  |            |
Runtime Resources Networking     (reached through ports)
```

Key architectural rules this spec follows:

1. **The API** It receives a request, validates its shape, translates it into the application's input, calls exactly one application use case and translates the result or error back. It holds no business logic and no state.
2. **Applications own the use cases.** Each API operation maps to one use case in the application directory (mapping in section 11). The use cases orchestrate the replica lifecycle.
3. **Execution coordinates the three subsystems** (Runtime, Resources, Networking) through ports. The API never talks to a subsystem directly.
4. **Dependencies point inward.** Application logic depends on ports (interfaces); infrastructure implements them.
5. **The Data Plane does not do service discovery or DNS.** Those belong to the Control Plane. The Data Plane only configures the network environment a job needs so it can take part in the Control Plane's service discovery.

The lifecycle the API drives (from the Data Plane README):

```text
Start -> Prepare (Resources, Networking, Runtime) -> Run
      -> Monitor (Resources, Networking, Runtime)
      -> Stop -> Cleanup
```

The Runtime sub-operations are `configure`, `start`, `monitor` (a single observation), `stop`, `cleanup`, and the Runtime reports `running`, `exited`, `signaled`, or `not found`.

---

## 3. Terminology

| Term | Meaning |
|---|---|
| **Job** | The unit of work the Control Plane assigns to a worker: a program plus its resource and network requirements. Used throughout this spec because the issue uses it. |
| **Replica** | The README's word for a running instance of work. This spec treats one job on a worker as one replica. Confirm the naming (D1). |
| **Worker** | The machine or host the Data Plane runs on. One Data Plane instance serves one worker. |
| **Desired state / spec** | What the caller asked for (`JobSpec`). |
| **Observed state** | What the Data Plane actually sees (process state, measured usage). |
| **Caller** | Whoever invokes the API. Today that is the Worker Client on behalf of the Control Plane. |
| **Request ID** | Caller-supplied identifier for one API call, used for tracing and safe retries. |

---

## 4. Conventions

| Topic | Convention |
|---|---|
| Operation names | camelCase as in the issue (`startJob`). |
| Identifiers | `job_id` is an opaque, non-empty string, 1 to 128 chars, `[A-Za-z0-9._-]`. **Assigned by the caller** (the Control Plane) so that retries are idempotent (D2). |
| Timestamps | UTC, RFC 3339 (for example `2026-10-21T14:03:22Z`). |
| CPU | Millicores (1000 = one core). |
| Memory | Bytes. |
| Durations | Whole seconds unless the field name says otherwise. |
| Pagination | Cursor-based: `page_size` and opaque `page_token`. |
| Sync vs async | Mutating operations return when the requested transition has been **accepted and its first step completed** (see each operation). Final outcomes are read with `getJob`. |
| Tracing | Every request carries an optional `request_id`; it is echoed in the response and in logs. |
| Versioning | The API carries a version number (`v1`). Unknown fields are ignored on read, and removed or changed fields require a new version. |

---

## 5. Domain Model

### 5.1 JobSpec (desired state)

| Field | Type | Required | Rules |
|---|---|---|---|
| `job_id` | string | yes | Per section 4. Unique on this worker. |
| `name` | string | no | Human-readable label, up to 128 chars. |
| `program` | string | yes | Executable path or command. Non-empty. |
| `args` | list of string | no | Up to 256 entries, each up to 4096 bytes. |
| `env` | map string to string | no | Keys match `[A-Za-z_][A-Za-z0-9_]*`. Total size up to 128 KiB. |
| `working_dir` | string | no | Absolute path. Defaults to a Runtime-chosen directory. |
| `resources.cpu_millicores` | integer | yes | Greater than 0, at most the worker's total. |
| `resources.memory_bytes` | integer | yes | Greater than 0, at most the worker's total. |
| `network` | NetworkSpec | no | See below. |
| `labels` | map string to string | no | Up to 32 entries. Informational only. |
| `stop_policy.grace_period_seconds` | integer | no | 0 to 300. Default 30. Time between the polite stop signal and the forced kill. |

**NetworkSpec** (shape is a placeholder until the Networking area finalizes it, D5): `ports` (list of container-side port numbers and protocol), `hostname` (optional), `service_name` (optional, only used to tell the Control Plane's discovery what to register).

### 5.2 JobState (observed lifecycle state)

| State | Meaning |
|---|---|
| `PREPARING` | Resources, networking, and runtime configuration are being set up. |
| `RUNNING` | The process is alive. |
| `STOPPING` | A stop was accepted and the process is being terminated. |
| `STOPPED` | The job ended because the caller asked it to stop. |
| `EXITED` | The process ended by itself with exit code 0. |
| `FAILED` | Preparation or start failed, or the process exited non-zero, or it was killed by a signal it was not sent by `stopJob`. |

Each job also reports `termination` once finished: `exit_code` (if exited), `signal` (if signaled), `reason` (one of `STOP_REQUESTED`, `COMPLETED`, `NONZERO_EXIT`, `SIGNALED`, `PREPARE_FAILED`, `START_FAILED`, `OOM_KILLED`, `WORKER_SHUTDOWN`), and `message`.

**Mapping from Runtime state** (the Runtime reports facts, the application decides the meaning):

| Runtime state | Job state |
|---|---|
| running | `RUNNING` |
| exited, code 0 | `EXITED` (or `STOPPED` if a stop was requested) |
| exited, code not 0 | `FAILED` (or `STOPPED` if a stop was requested) |
| signaled | `FAILED`, reason `SIGNALED` or `OOM_KILLED` (or `STOPPED` if a stop was requested) |
| not found | `FAILED` with reason unknown, flagged for the caller (D8) |

### 5.3 Lifecycle transitions

| From | Operation or event | To | Notes |
|---|---|---|---|
| (none) | `startJob` | `PREPARING` then `RUNNING` | If any prepare step fails, rollback then `FAILED`. |
| `RUNNING` | `stopJob` | `STOPPING` then `STOPPED` | |
| `RUNNING` | process exits | `EXITED` or `FAILED` | Detected by the monitoring loop in Execution. |
| `STOPPING` | process ends | `STOPPED` | |
| `PREPARING` | `stopJob` | `STOPPED` | Abort preparation, roll back. |
| `STOPPED`, `EXITED`, `FAILED` | `deleteJob` | (removed) | Runs cleanup. |
| any | `getJob` and other reads | unchanged | Reads never change state. |

Anything not in this table is an **illegal transition** and returns `FAILED_PRECONDITION` (section 8).

### 5.4 Resource views

**WorkerResources** (`getResources`): `total` and `allocated` and `available`, each with `cpu_millicores` and `memory_bytes`, plus `job_count`, `measured_at`. Invariant: `available = total - allocated`, never negative.

**JobResources** (`getJobResources`): `job_id`, `allocated` (what was reserved), `usage` (current measured CPU millicores and memory bytes), `peak_memory_bytes`, `cpu_throttled` (boolean), `measured_at`. Usage is only meaningful while `RUNNING`; for finished jobs it returns the last recorded values with `final: true`.

---

## 6. Common Request and Response Rules

**Every request** may include `request_id` (string, up to 64 chars). **Every response** includes `request_id` (echoed) and either a result or an error.

**Error object**

| Field | Meaning |
|---|---|
| `code` | One of the codes in section 8. |
| `message` | Human-readable explanation. Never contains secrets such as environment values. |
| `retryable` | Boolean hint: may the caller retry unchanged? |
| `job_id` | The job concerned, if any. |
| `subsystem` | `api`, `application`, `runtime`, `resources`, or `networking`, where the error originated. |
| `details` | Optional structured data, for example a list of invalid fields. |

**Validation layers**

1. *API layer*: structure and types, required fields, sizes, formats (cheap checks, no state needed).
2. *Application layer*: rules that need state (does the job exist, is the transition legal, is there enough capacity).
3. *Subsystems*: rules only they know (can this port be bound, is this path usable).

All validation errors are reported together where possible (`details` lists every invalid field), not one at a time.

---

## 7. Operations

Each operation lists: purpose, responsibilities, input, output, validation, errors, idempotency, failure behavior.

### 7.1 `startJob`

**Purpose:** Accept a job and run it on this worker.

**Responsibilities**

- Validate the spec and check capacity.
- Reserve resources, configure networking, configure the runtime, then start the process (Prepare then Run).
- Record the job so later operations can find it.
- Hand the job to the monitoring loop.

**Input:** `JobSpec` (section 5.1), optional `request_id`.

**Output:** `job_id`, `state` (`RUNNING` on success), `created_at`, `started_at`, `allocated` resources, `network` info (assigned addresses and ports, if any).

**Validation:** all `JobSpec` rules; `job_id` must not belong to another live job with a different spec; requested resources must fit in `available`; worker must not be draining.

**Errors:** `INVALID_ARGUMENT`, `ALREADY_EXISTS` (same `job_id`, different spec), `RESOURCE_EXHAUSTED`, `UNAVAILABLE` (draining or shutting down), `DEADLINE_EXCEEDED`, `INTERNAL` (with `subsystem` and a `phase` of `resources`, `networking`, `runtime.configure`, `runtime.start`).

**Idempotency:** same `job_id` and identical spec returns the existing job (success, not an error). A job that is already `RUNNING` is returned as is.

**Failure behavior**

- Steps run in order resources, networking, runtime configure, runtime start. On failure at step N, steps N-1 down to 1 are **undone in reverse order**, the job is recorded as `FAILED` with reason `PREPARE_FAILED` or `START_FAILED`, and no resources stay reserved.
- If rollback itself fails, the job is recorded as `FAILED` with `cleanup_pending: true` and the failure is logged and counted. The caller can retry via `deleteJob`.
- A timeout (D6) is treated as a failure of the step in progress.
- The operation must never leave capacity reserved for a job that is not running.

---

### 7.2 `stopJob`

**Purpose:** Terminate a job.

**Responsibilities:** Signal the process politely, wait up to the grace period, then force-kill; record the outcome. Resources and network setup are **not** released here, that is `deleteJob` (D3 asks whether they should be).

**Input:** `job_id`, optional `grace_period_seconds` (overrides the spec's `stop_policy`), optional `force` (boolean, skip the grace period).

**Output:** `job_id`, `state` (`STOPPING` or `STOPPED`), `termination` (once available).

**Validation:** `job_id` well-formed; grace period 0 to 300.

**Errors:** `INVALID_ARGUMENT`, `NOT_FOUND`, `FAILED_PRECONDITION` (for example the job is `PREPARING` and cannot yet be signaled, if D7 is decided that way), `DEADLINE_EXCEEDED`, `INTERNAL`.

**Idempotency:** stopping a job that is `STOPPING`, `STOPPED`, `EXITED`, or `FAILED` succeeds and returns the current state. Stopping an unknown job returns `NOT_FOUND`.

**Failure behavior:** if the polite signal fails, escalate to a forced kill. If the process cannot be killed, the job stays `STOPPING`, the error is `INTERNAL` with `retryable: true`, and the event is logged and counted. A job that finished on its own during the call is reported with its real outcome (`EXITED` or `FAILED`), not rewritten as `STOPPED`.

---

### 7.3 `updateJob`

**Purpose:** Change a job that already exists without replacing it.

**Responsibilities:** Apply the allowed changes atomically. Anything not on the allowed list is rejected.

**Mutable fields (proposed):** `resources` (cpu and memory limits), `labels`, `stop_policy`. **Immutable:** `program`, `args`, `env`, `working_dir`, `network`. To change those the caller stops the job and starts a new one (D4).

**Input:** `job_id`, an `update` object containing only the fields to change, optional `expected_revision` (integer, for optimistic concurrency, D9).

**Output:** the updated job view (same shape as `getJob`) with an incremented `revision`.

**Validation:** at least one field present; no immutable field present; new resources must fit (`available + current allocation >= requested`); job must be `RUNNING` for resource changes (labels and stop policy may change in any non-removed state).

**Errors:** `INVALID_ARGUMENT`, `NOT_FOUND`, `FAILED_PRECONDITION` (immutable field, or wrong state), `RESOURCE_EXHAUSTED`, `CONFLICT` (`expected_revision` mismatch), `INTERNAL`.

**Idempotency:** applying an update that already matches the current spec succeeds with no change.

**Failure behavior:** all or nothing. If the resource limit cannot be applied to the running process, the previous allocation is restored and the revision does not change.

---

### 7.4 `getJob`

**Purpose:** Return one job's spec, observed state, and termination details.

**Responsibilities:** Read-only. Combine the stored spec with a fresh or recently cached observation of the process (via `monitor`).

**Input:** `job_id`.

**Output:** `job_id`, `spec` (with `env` values **redacted**, keys only, D10), `state`, `termination`, `revision`, `created_at`, `started_at`, `finished_at`, `network` info, `cleanup_pending`, `observed_at`.

**Validation:** `job_id` well-formed.

**Errors:** `INVALID_ARGUMENT`, `NOT_FOUND`, `INTERNAL`.

**Idempotency:** naturally idempotent, no side effects.

**Failure behavior:** if a live observation fails, return the last known state with `observed_at` showing its age and `stale: true`, rather than failing the read. If the runtime reports `not found` for a job the Data Plane believes is running, report `FAILED` and flag the inconsistency (D8).

---

### 7.5 `listJobs`

**Purpose:** List jobs on this worker, optionally filtered.

**Input:** optional `states` (list of `JobState`), optional `labels` (exact-match filter), `page_size` (1 to 500, default 100), `page_token`.

**Output:** `jobs` (list of job summaries: `job_id`, `name`, `state`, `created_at`, `allocated`), `next_page_token` (absent on the last page), `total_estimate` (optional).

**Validation:** `page_size` range; `page_token` must be one this API issued; filter values well-formed.

**Errors:** `INVALID_ARGUMENT` (including an expired or malformed token), `INTERNAL`.

**Idempotency:** read-only.

**Failure behavior:** results come from a consistent snapshot per page. Jobs created or removed between pages may be missed or repeated across pages, but never duplicated within a page. Ordering is stable: by `created_at`, then `job_id`.

---

### 7.6 `deleteJob` (supporting operation, runs Cleanup)

**Purpose:** Remove everything the Data Plane holds for a finished job: runtime state, reserved resources, network setup, and the job record.

**Input:** `job_id`, optional `force` (boolean: stop first if still running, D3).

**Output:** `job_id`, `deleted: true`.

**Errors:** `NOT_FOUND` (see idempotency), `FAILED_PRECONDITION` (job still `RUNNING` or `STOPPING` and `force` is false), `INTERNAL` (with `cleanup_pending`).

**Idempotency:** deleting a job that no longer exists returns success with `deleted: true` (D11 asks whether `NOT_FOUND` is better).

**Failure behavior:** cleanup is **best effort per subsystem and continues past failures**: a failure in networking cleanup does not skip resource release. Any failed step is reported in `details`, the job remains with `cleanup_pending: true`, and calling `deleteJob` again retries only the unfinished steps. Leaks of processes, reserved capacity, or runtime state are treated as bugs and must be visible in metrics.

---

### 7.7 `getResources`

**Purpose:** Report worker capacity and availability so the Control Plane can schedule.

**Input:** none (optionally `request_id`).

**Output:** `WorkerResources` (section 5.4).

**Errors:** `UNAVAILABLE` (resource subsystem cannot read the host), `INTERNAL`.

**Idempotency:** read-only.

**Failure behavior:** `available` is computed from allocations the Data Plane has recorded, so it stays correct even if live measurement fails. If live host measurement fails, return the recorded values with `stale: true`. The numbers are a point-in-time snapshot, so a later `startJob` can still return `RESOURCE_EXHAUSTED` and callers must handle it.

---

### 7.8 `getJobResources`

**Purpose:** Report one job's allocation and current usage.

**Input:** `job_id`.

**Output:** `JobResources` (section 5.4).

**Errors:** `INVALID_ARGUMENT`, `NOT_FOUND`, `INTERNAL`.

**Idempotency:** read-only.

**Failure behavior:** if usage cannot be measured, return `allocated` with `usage` absent and `stale: true`. For a finished job, return final recorded values.

---

### 7.9 `getHealth` (supporting operation)

**Purpose:** Let the caller know the Data Plane is alive and able to accept work.

**Output:** `status` (`READY`, `DRAINING`, `DEGRADED`, `UNAVAILABLE`), per-subsystem status for `runtime`, `resources`, `networking`, `version`, `uptime_seconds`.

**Behavior:** cheap, never touches job state, answers even when a subsystem is degraded. `DRAINING` means no new `startJob` is accepted.

---

### 7.10 Other candidate operations ("and more")

The issue leaves room for more operations. These are **proposed, not committed**; each needs a decision (section 12).

| Operation | Purpose | Note |
|---|---|---|
| `getJobNetwork` | Current network configuration and connectivity for a job. | Depends on the Networking area (D5). |
| `getJobLogs` | Return or stream a job's stdout/stderr. | Large design question, may belong in a later version (D13). |
| `restartJob` | Stop then start with the same spec. | Likely a Control Plane decision composed from `stopJob` and `startJob` (D14). |
| `drainWorker` | Stop accepting new jobs. | Supports maintenance, pairs with `getHealth` status `DRAINING`. |
| `watchJobs` | Push state changes instead of polling. | Depends on transport, out of scope for now. |

---

## 8. Error Catalog

| Code | When | Retryable |
|---|---|---|
| `INVALID_ARGUMENT` | Malformed or out-of-range input. `details` lists every invalid field. | No, fix the request. |
| `NOT_FOUND` | `job_id` unknown on this worker. | No. |
| `ALREADY_EXISTS` | `job_id` exists with a different spec. | No. |
| `FAILED_PRECONDITION` | Operation is illegal in the job's current state, or an immutable field was changed. | No, until state changes. |
| `RESOURCE_EXHAUSTED` | Not enough CPU or memory. | Yes, after capacity changes or on another worker. |
| `CONFLICT` | Another operation on the same job is in progress, or `expected_revision` is stale. | Yes, after re-reading. |
| `UNAVAILABLE` | Worker draining, shutting down, or a subsystem is down. | Yes, with backoff. |
| `DEADLINE_EXCEEDED` | An internal step exceeded its time limit. | Depends on the operation, see idempotency. |
| `INTERNAL` | Unexpected subsystem failure. `subsystem` and `phase` are set. | Maybe, see `retryable`. |

Rules: errors never expose secrets; every error is logged with `request_id`, `job_id`, `code`, and `subsystem`; the `retryable` flag is set by the application layer, not guessed by the caller.

---

## 9. Concurrency and Idempotency

- **One mutating operation per job at a time.** `startJob`, `stopJob`, `updateJob`, and `deleteJob` on the same `job_id` are serialized. A second one arriving while the first is in flight either waits briefly or returns `CONFLICT` (D9).
- **Reads never block on mutations.** They return the last committed state, marked with `observed_at`.
- **Operations on different jobs run in parallel.**
- **Capacity checks and reservations are atomic**, so two concurrent `startJob` calls can never both succeed on capacity that fits only one.
- **Retries are safe** for every operation as described in each idempotency note. The caller may retry after a timeout without knowing whether the first attempt landed.
- **Monitoring is not an API operation.** `monitor` takes a single observation. The loop that calls it periodically lives in Execution, so job state changes (a process exiting on its own) are recorded even when nobody is calling the API.

---

## 10. Failure Behavior Summary

| Scenario | Required behavior |
|---|---|
| Prepare step fails mid-start | Roll back completed steps in reverse, record `FAILED`, no reserved capacity left. |
| Process crashes | Monitoring loop records `FAILED` with exit code or signal. Resources stay reserved until `deleteJob` (D3). |
| Process ignores polite stop | Force kill after grace period. |
| Rollback or cleanup step fails | Job kept with `cleanup_pending: true`, retry via `deleteJob`. |
| Subsystem unavailable | Reads return stale data flagged `stale: true`. Mutations return `UNAVAILABLE`. |
| Data Plane restarts | Open decision D8: either rebuild job state by reconciling persisted records with the runtime, or report all prior jobs as unknown and let the Control Plane reconcile. |
| Duplicate or retried request | Handled by the idempotency rules above, never creates a second process. |
| Worker shutting down | Switch to `DRAINING`, reject `startJob`, finish in-flight operations, stop jobs per policy with reason `WORKER_SHUTDOWN`. |

**Observability requirement (all operations):** log every call with `request_id`, `job_id`, operation, outcome, and latency; count calls by operation and error code; record latency histograms per operation; expose gauges for jobs by state, allocated resources, and `cleanup_pending` jobs.

---

## 11. Mapping to the Application Layer

The application directory holds the use cases and is what the API facade calls. Proposed one-to-one mapping (names are proposals, align with the existing use-case naming convention):

| API operation | Application use case | Subsystems touched |
|---|---|---|
| `startJob` | start job | Resources, Networking, Runtime (`configure`, `start`) |
| `stopJob` | stop job | Runtime (`stop`) |
| `updateJob` | update job | Resources (and the job record) |
| `getJob` | get job | Runtime (`monitor`), job record |
| `listJobs` | list jobs | job record |
| `deleteJob` | delete (cleanup) job | Runtime (`cleanup`), Resources, Networking |
| `getResources` | get worker resources | Resources |
| `getJobResources` | get job resources | Resources |
| `getHealth` | get health | all (status only) |

The facade in `api.rs` constructs these use cases (through the bootstrap wiring) and exposes one entry point per operation. Request and response types for the facade live at the API boundary and are converted to domain types, so domain models are never leaked to callers directly.

---

## 12. Assumptions Requiring Review

The assumptions below make this draft concrete, but they are **provisional and not approved requirements**. Review each with someone else before implementation. Update later.

| ID | Assumption to review | Alternatives | Proposed working assumption (not yet approved) |
|---|---|---|---|
| D1 | Job vs replica naming. | Use `job` everywhere, or `replica` everywhere, or both with a defined mapping. | Pick one for the API and document the other as an alias. |
| D2 | Who assigns `job_id`? | Caller, or Data Plane. | Caller, so retries are idempotent. |
| D3 | Does `stopJob` release resources, or only `deleteJob`? | Release on stop, or keep until delete. | Keep until `deleteJob` so exit details and final usage stay inspectable, but add a retention limit to avoid leaks. |
| D4 | What can `updateJob` change? | Resources only, or also env, args. | Resources, labels, stop policy only. |
| D5 | Final `NetworkSpec` shape. | Depends on the Networking area. | Keep it a placeholder until Networking defines ports, addressing, and hostnames. |
| D6 | Timeouts per operation. | Fixed defaults, or caller-supplied. | Fixed defaults per step, documented, plus an overall deadline. |
| D7 | `stopJob` while `PREPARING`. | Abort preparation, or reject. | Abort and roll back. |
| D8 | State persistence and restart recovery. | In-memory only, or persisted records reconciled with the Runtime on boot. | Persisted records plus reconciliation; also decide what `not found` from the Runtime means. |
| D9 | Concurrent mutations on one job. | Queue them, or return `CONFLICT`. Is `revision` / `expected_revision` needed? | Return `CONFLICT` and keep `revision`. |
| D10 | Secrets in `env`. | Redact values on read, or return as stored. | Redact on read. |
| D11 | `deleteJob` on an unknown job. | Success or `NOT_FOUND`. | Success (idempotent). |
| D12 | Directory name `application/` vs `applications/`. | Pick one. | Match the repository's actual folder. |
| D13 | Logs. | In this API, or a separate facility. | Defer. |
| D14 | Restart semantics. | Data Plane operation, or Control Plane composition. | Control Plane composes stop and start. |
| D15 | Who restarts a crashed job? | Data Plane restart policy, or Control Plane reconciliation. | Control Plane (it owns desired state). The Data Plane only reports. |
| D16 | Limits. | Max jobs per worker, max concurrent mutations. | Define as configuration and report them in `getResources`. |

---

## 13. Appendix: Illustrative Payloads (documentation only)

**startJob request**

```json
{
  "request_id": "req-8f3a",
  "job_id": "job-0042",
  "name": "hello-server",
  "program": "/usr/bin/python3",
  "args": ["-m", "http.server", "8080"],
  "env": {"MODE": "test"},
  "working_dir": "/srv/app",
  "resources": {"cpu_millicores": 500, "memory_bytes": 268435456},
  "network": {"ports": [{"port": 8080, "protocol": "tcp"}]},
  "stop_policy": {"grace_period_seconds": 20}
}
```

**startJob response (success)**

```json
{
  "request_id": "req-8f3a",
  "job_id": "job-0042",
  "state": "RUNNING",
  "created_at": "2026-10-21T14:03:22Z",
  "started_at": "2026-10-21T14:03:23Z",
  "allocated": {"cpu_millicores": 500, "memory_bytes": 268435456}
}
```

**Error response**

```json
{
  "request_id": "req-8f3a",
  "error": {
    "code": "RESOURCE_EXHAUSTED",
    "message": "Requested 500 millicores, 200 available.",
    "retryable": true,
    "job_id": "job-0042",
    "subsystem": "resources"
  }
}
```

---