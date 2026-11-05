# Client-Control Plane API Specification

**Status:** Draft for team review | **Issue:** [#26](https://github.com/2000023946/GT-Cloud/issues/26)

This is a documentation-only proposal. It introduces no implementation code and
includes fields and behavior not yet supported by existing models. Author decisions
are identified explicitly; other defaults are proposals for team review.

## 1. Scope and operation coverage

[Issue #26](https://github.com/2000023946/GT-Cloud/issues/26) requires client–control-plane
operations, models, resource/status queries, errors, authentication, and communication
flows, with no implementation code. Worker transport and scheduling/networking/
observability internals remain outside this document.

The issue explicitly requests CRUD and redeploy for systems, services, and deployments.
**Reviewer agreement required:** this draft proposes service create/update/delete
through complete system uploads, deployment creation/deletion with its service,
and service redeploy through its deployment. The service read-only API does not
independently satisfy that wording unless reviewers approve this hierarchical mapping.
Otherwise direct operations need a follow-up revision before the spec is ready.

## 2. Concepts and workloads

`User -> System -> Service -> Deployment -> Replica -> Worker`

A system groups applications submitted together. A service describes application
code/command, per-replica resources, networking and workload type. Its deployment
owns desired scale and execution revision. Replicas are individual execution attempts
placed on workers. Resource requests describe allocation; usage describes observation.

**Author decision:** support both long-running services and one-shot jobs. Add
`workloadType: SERVICE | JOB` to service responses and the eventual configuration
schema. Systems remain the submission unit. A job is a finite workload inside a
system, not a synonym for any running service. One deployment belongs to each service.
The existing client Job model must be adapted to this hierarchy.

**Proposed job contract, requiring worker/reconciliation owner agreement:**

* For JOB, desiredReplicas is the number of independent executions in one batch.
  All requested executions must exit successfully for COMPLETED. Successful attempts
  are not replaced. Autoscaling is unsupported for jobs.
* V1 does not automatically retry failed/lost job attempts. Remaining attempts may
  settle, then the batch is FAILED. Explicit redeploy starts a new batch, rerunning
  even previously successful work. Exactly-once execution is not guaranteed: worker
  loss can leave side effects with an uncertain result.
* After a batch starts, positive-to-positive count changes return 409. Stop is allowed;
  resume from zero starts a new revision/batch. Code/config changes and explicit
  redeploy start a new batch. Before execution starts, count changes are allowed.
* Unexpected SERVICE exit, including exit zero, is failure and triggers replacement
  with a new replica ID. Successful JOB exit produces COMPLETED.

## 3. Transport, authentication, and errors

Proposed transport: HTTP/JSON under /v1, camelCase field names, UPPER_SNAKE_CASE enums,
opaque server-assigned IDs, RFC 3339 UTC timestamps. System uploads use multipart/form-data.
Unknown request fields are rejected; clients ignore unknown response fields. Breaking
changes require a new API version. Nullable fields are present as null; final page
tokens are omitted. Examples are illustrative, not exhaustive schemas.

Proposed authentication: Authorization: Bearer <token> on every request. Server derives
ownership from credentials, never input. Users may access only their systems and
descendants, including logs/metrics. Lists are ownership-filtered before pagination.
Worker/capacity summaries are readable by any authenticated user without management
addresses. Token issuance/expiry and GT SSO belong to a separate auth contract.

All errors use one body:

```json
{
  "error": {
    "code": "VALIDATION_FAILED",
    "message": "desiredReplicas must be between 0 and 100",
    "details": [
      { "field": "desiredReplicas", "reason": "out_of_range" }
    ],
    "requestId": "req_8f2c"
  }
}
```

| HTTP | `code` | When |
| --- | --- | --- |
| 400 | `VALIDATION_FAILED` | Malformed or invalid request |
| 401 | `UNAUTHENTICATED` | Missing or invalid credentials |
| 403 | `FORBIDDEN` | Authenticated, but not allowed |
| 404 | `NOT_FOUND` | Resource does not exist (or is not visible to the caller) |
| 409 | `CONFLICT` | Name already exists, or state does not allow the operation |
| 413 | `PAYLOAD_TOO_LARGE` | ZIP exceeds the size limit |
| 422 | `UNPROCESSABLE_SYSTEM` | ZIP uploaded but code/config could not be parsed |
| 429 | `RATE_LIMITED` | Too many requests |
| 500 | `INTERNAL` | Unexpected control plane failure |
| 503 | `UNAVAILABLE` | Control plane dependency (e.g., storage) unavailable; retryable |

`404` is returned instead of `403` for resources owned by another user, so
resource existence is not leaked.

## 4. API operations and resource responses

### 4.1 Systems

| Operation | Method and path | Success |
| --- | --- | --- |
| Create | `POST /v1/systems` | `202` + system |
| List | `GET /v1/systems` | `200` + page of systems |
| Get | `GET /v1/systems/{systemId}` | `200` + system |
| Update | `PUT /v1/systems/{systemId}` | `202` + system |
| Delete | `DELETE /v1/systems/{systemId}` | `202` |
| Redeploy | `POST /v1/systems/{systemId}:redeploy` | `202` + system |

Create additionally returns Location: /v1/systems/{id}. Delete returns the system
with DELETING status. Update replaces complete uploaded input, subject to preserved
scaling settings in section 6. Actions accept no body.

System responses contain id, name, userId, createdAt, updatedAt, services (IDs), and
status. Create requires name/code/config multipart parts; update requires code/config.

### 4.2 Services

| Operation | Method and path | Success |
| --- | --- | --- |
| List (all) | `GET /v1/services?systemId={id}` | `200` + page |
| Get | `GET /v1/services/{serviceId}` | `200` + service |

Service responses contain id, name, systemId, workloadType, app {command},
resources {cpu, memory}, network [{protocol, port, direction}], and deploymentId.
Do not expose internal app.codePath. New resources require positive allocations.
Protocol/port validation depends on the networking contract; TCP/UDP and integer
ports 1-65535 are proposed. CPU representation remains unresolved (section 10).

### 4.3 Deployments

| Operation | Method and path | Success |
| --- | --- | --- |
| List | `GET /v1/deployments?serviceId={id}` | `200` + page |
| Get | `GET /v1/deployments/{deploymentId}` | `200` + deployment |
| Update | `PATCH /v1/deployments/{deploymentId}` | `202` + deployment |
| Redeploy | `POST /v1/deployments/{deploymentId}:redeploy` | `202` + deployment |
| Stop | `POST /v1/deployments/{deploymentId}:stop` | `202` + deployment |

Deployment responses contain id, serviceId, desiredReplicas, autoscale
{enabled, minReplicas, maxReplicas, cpuThreshold}, generation, observedGeneration,
revision, converged, status, reason, message, and replicas
{desired, pending, running, completed, failed, stopped}.
PATCH accepts only desiredReplicas and autoscale. Counts are integers; proposed
limit is 0-100 desired replicas. Autoscale threshold is in (0,1]; its denominator
requires monitoring agreement. Lifecycle and merge rules are in section 6.

### 4.4 Replicas

| Operation | Method and path | Success |
| --- | --- | --- |
| List | `GET /v1/replicas?deploymentId={id}` | `200` + page |
| Get | `GET /v1/replicas/{replicaId}` | `200` + replica |

Replica responses contain id, deploymentId, revision, workerId, status,
endpoints [{address, port, protocol}], startedAt, finishedAt, exitCode, reason,
and message. Nullability and status rules are in section 7. No restartCount is
exposed because each execution attempt receives a new replica ID.

### 4.5 Workers and capacity

| Operation | Method and path | Success |
| --- | --- | --- |
| List | `GET /v1/workers` | `200` + page |
| Get | `GET /v1/workers/{workerId}` | `200` + worker |
| Cluster capacity | `GET /v1/capacity` | `200` + summary |

Worker responses contain id, status, timestamp, capacity {cpu,memory}, allocated
{cpu,memory}, and replicaCount. Status is UNKNOWN, READY, BUSY, or OFFLINE.
Capacity responses contain timestamp, capacity, allocated, and available, each
resource object containing cpu/memory. See freshness/allocation rules in section 9.

### 4.6 Usage

GET /v1/replicas/{id}/usage, GET /v1/deployments/{id}/usage, and
GET /v1/services/{id}/usage return 200 with the latest measurement envelope:

```json
{
  "scope": "DEPLOYMENT",
  "id": "dep_1",
  "timestamp": null,
  "complete": false,
  "expectedReplicas": 2,
  "measuredReplicas": 1,
  "cpu": { "usage": null, "allocated": 2 },
  "memory": { "usage": null, "allocated": 2048 },
  "replicas": [
    { "replicaId": "rep_1", "timestamp": "2026-10-12T04:30:00Z", "stale": false, "cpu": { "usage": 0.5 }, "memory": { "usage": 512 } },
    { "replicaId": "rep_2", "timestamp": null, "stale": false, "cpu": { "usage": null }, "memory": { "usage": null } }
  ]
}
```

Usage is an instantaneous/latest query, not a historical series. Section 9 defines
aggregation and missing data. CPU numbers are placeholders until units are agreed.

### 4.7 Logs and metrics

Reserved routes: GET /v1/deployments/{id}/logs,
GET /v1/replicas/{id}/logs, GET /v1/deployments/{id}/metrics.
Ownership checks happen before querying/proxying backends. Shapes, time-range filters,
pagination, streaming and retention depend on #29/#30. These routes are not ready
for implementation until those contracts are linked here. Workload-status polling
does not preclude a separate live-log streaming contract.

## 5. Uploads and request validation

**Author-accepted configurable defaults:** 50 MiB compressed per archive, 500 MiB
total extracted across both archives, 10,000 total entries including directories.
One MiB is 1,048,576 bytes. Enforce limits while reading/extracting; return 413 on
any limit violation. Return 422 for corrupt archives, absolute/traversing paths,
symbolic links, or duplicate normalized paths.

Require exactly one code and config part, plus name on create. Reject duplicate/
unknown parts with 400. Proposed names match `[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?`.
Names are immutable; reject a name part on update. At least one uniquely named
service is required. The archive-format/config-schema specification must define
layout, filenames, relative code references, workload types, and defaults. No such
contract was identified in this checkout; this remains a separate required dependency.

SubmitSystemRequest needs name and authenticated-owner context; UpdateSystemRequest
needs target system ID and owner context. Path supplies the ID, authentication
supplies ownership; uploaded input cannot assign ownership. The current types do
not provide a complete mapping of this API.

## 6. Updates, stop, and concurrent changes

**Author decisions:**

* Match services by name. Retained names keep service/deployment IDs; additions get
  new IDs; removals shut down and are removed after cleanup. Renames remove/create.
* Preserve current deployment replica count and all autoscaling settings during an
  upload, even if uploaded defaults differ. Defaults initialize new deployments only.
* Stop all old replicas of an affected deployment before starting replacements;
  downtime is allowed. Shutdown must be confirmed, or old execution safely isolated,
  before replacement. Shutdown failure leaves the deployment unconverged with a reason.
* Stop atomically disables autoscaling and sets count zero. Other autoscale fields
  are preserved. Resume by setting a positive count; explicitly re-enable autoscaling
  if wanted. Repeated stop is harmless and does not repeatedly increment generation.
* Order accepted mutations per system, including its deployments. Last successfully
  validated and durably accepted write wins for overlapping fields. Failed requests
  never supersede accepted input; stale revisions are not rejected in V1.

**Additional proposed details:** unchanged code/config retains execution revision;
a changed code archive may conservatively affect all services. Existing workload
type is immutable for a retained name; changing it requires removal followed by
addition in a later update. System replacement validates the entire new input
before acceptance. Retained scaling settings also apply to completed jobs.

Redeploy uses latest accepted code/config and starts a fresh revision, preserving
scale. Completed/failed jobs start a fresh batch. Stopped deployments remain stopped;
redeploy of a stopped deployment or entirely stopped system returns 409. Repeated
redeploy can restart work again; clients inspect state before retrying uncertain actions.

Delete is terminal once accepted. Update/scale/stop/redeploy during deletion returns
409. Reads remain available as DELETING until cleanup finishes, then return 404.
Name uniqueness remains reserved until deletion completes. Repeated delete during
cleanup returns 202; after removal it returns 404. Removed services remain visible
as deleting until cleaned up.

PATCH merges nested autoscale fields, preserves omitted fields, and rejects null,
unknown fields, and empty patches. Validate the resulting object atomically. Manual
scale while autoscaling remains enabled returns 409; disable it in the same PATCH
or use stop. Proposed new-deployment defaults: one replica, autoscale disabled,
min 1, max 100, CPU threshold 0.7. Min/max are integers 1–100 with min <= max.

## 7. Acceptance, progress, and statuses

202 guarantees durable desired state and a recoverable obligation to reconcile,
including after restart. No partially accepted system/configuration is exposed on
failure before acceptance. Invalid or failed replacement preserves previous accepted
state. Staged internal artifacts may be cleaned up later. Persistence/queue mechanics
are outside this API specification.

Deployments add `revision`, `converged`, and nullable `reason`/`message` to the existing
response. Generation increments on accepted desired-state changes. Revision increments
on execution replacement/redeploy, not ordinary service scaling. Replicas carry
`revision`. Counts describe the current revision; old replicas remain listable.

Generation equality means processed, not successful. Converged requires latest
generation processed, no superseded active replicas, and target satisfaction:
SERVICE has desired current replicas running; JOB has all requested executions
completed; stopped deployment has no pending/running replicas. Clients poll
convergence or failure, not generation equality. Suggested polling is two seconds,
backing off to ten; a client timeout never implies server rollback.

Replica status: PENDING -> RUNNING -> COMPLETED | FAILED | STOPPED. Pending attempts
may fail or stop. Each attempt gets a new ID; replace restartCount with nullable
finishedAt/exitCode/reason/message. workerId is nullable before placement and startedAt
before execution. Replace endpoint with `endpoints: [{address, port, protocol}]`, empty
until known, to support multiple ports. Addresses do not guarantee public reachability.

Evaluate deployment rules in order; active means pending/running, current means
current revision. Historical failures do not poison a new revision.

| Status | Rule |
| --- | --- |
| DELETING | Parent/service removal accepted; cleanup outstanding |
| STOPPING | Desired zero and active replicas remain, including old revisions |
| STOPPED | Desired zero and no active replicas remain |
| UPDATING | Old revision replicas remain active, or current running count exceeds desired |
| COMPLETED | JOB: all requested current executions succeeded |
| FAILED | JOB: a failure exists and no active attempts remain; SERVICE: no current running/pending replicas and a current failure is reported |
| RUNNING | SERVICE: desired current replicas running; JOB: all requested executions running/completed, at least one running, none failed |
| DEGRADED | Some current replicas running, but target not satisfied or a job attempt failed |
| PENDING | Remaining cases, including before replica records exist or capacity is unavailable |

Status describes execution; convergence additionally requires processing the latest
generation. Service failures may recover; failed jobs are terminal for that revision.
Example reasons: INSUFFICIENT_CAPACITY, START_FAILED, WORKER_LOST, PROCESS_EXITED,
STOP_FAILED. Clients tolerate unknown future reasons and never parse message text.

System deletion takes precedence (DELETING). Otherwise map deployment PENDING and
UPDATING to DEPLOYING, then select the first present status:
FAILED > DEGRADED > STOPPING > DEPLOYING > RUNNING > COMPLETED > STOPPED.
A running service plus completed job is RUNNING; completed jobs plus stopped services
is COMPLETED. Ignore fully removed services. Empty transitional systems remain
DEPLOYING until updates/removal settle. System responses therefore also permit
STOPPING and COMPLETED. Client COMPLETED maps to completed jobs, not an unsupported state.

## 8. Idempotency and retry behavior

**Author decision:** support idempotency keys. The following details are proposed.
The client sends a key; the control plane provides the retry guarantee.

POST /v1/systems requires Idempotency-Key, a client-generated opaque value of 1–128
printable ASCII characters per logical submission. Server scope is authenticated
user + method + path; accepted results are retained at least 24 hours. Equal name
and identical archive bytes mean equal input; multipart boundaries/filenames do not.

Same key/input replays original 202 body and Location without creating another system,
even if it later changes or is deleted. Different input returns 409 IDEMPOTENCY_CONFLICT;
an in-flight equal request returns 409 REQUEST_IN_PROGRESS with Retry-After seconds.
Authentication is still checked. Validation failures do not consume keys. Acceptance
and the replay record must be atomic from the client's perspective. After expiry,
name uniqueness prevents duplicates while the original exists but is not an unlimited
retry guarantee. Create returns `Location: /v1/systems/{id}`.

Other mutations do not accept keys in V1. Redeploy retries can create another batch;
inspect state first. Common errors additionally include 415 UNSUPPORTED_MEDIA_TYPE.
429/503 responses carry Retry-After seconds. Error code/message/details/requestId
are required; details may be an empty array. Never expose stack traces or credentials.

## 9. Resource observations, pagination, and browser access

CPU units/representation remain **unknown**; existing examples do not settle them.
Memory bytes are proposed. Agree CPU units, utilization denominator, averaging
window, and stale-data threshold with resource/monitoring owners before implementation.

Usage adds complete, expectedReplicas, measuredReplicas and per-replica timestamp/
stale fields. Missing/stale data never counts as zero: aggregate usage is null unless
all running replicas have fresh samples. Include old revisions still running during
shutdown. Aggregate timestamp is the oldest fresh sample, or null if incomplete.
No running replicas means null usage, zero counts, complete false. Allocated sums
reservations for placed replicas, not desired scale. Replica/service use the same
envelope with scope REPLICA/SERVICE; replica scope contains one entry. These rules
supersede the partial aggregation implied by the original usage example.

Capacity allocated means placed reservations not yet released, including pending
starts/stops. Available is capacity minus allocated across READY/BUSY workers; it
is advisory, not a placement guarantee. Add nullable observation timestamp to worker
and capacity responses. Freshness policy remains a monitoring dependency.

Lists default to ascending immutable ID. Parent filters are optional; omission
lists all caller-owned resources. Foreign/missing explicit parents return 404.
Reject unsupported filters and pageSize outside 1–200 with 400. Tokens bind caller
and filters; invalid/expired tokens return 400 with detail invalid_page_token.
Proposed lifetime is at least 15 minutes. Listing is not a snapshot under concurrent
changes; clients may restart/deduplicate by ID. Replica lists optionally filter by
positive-integer revision. Proposed replica retention is at least 24 hours after
termination while parent exists; parent deletion may remove records.

All resource/list/log/usage access is ownership checked before pagination or backend
queries. HTTPS is proposed outside local development. Web origin, CORS, and token
storage need agreement with the client/auth owner; same-origin hosting is not assumed.
Public service exposure is a separate networking contract. Protocol/port/direction
validation must align with that spec; replica endpoints alone are not public URLs.

## 10. Readiness, dependencies, and responsibilities

This is a documentation-only PR. Author decisions above resolve workload support,
service identity, stop-first update, scaling preservation, stop/autoscale interaction,
last accepted write wins, idempotency support, and upload-limit defaults.

Before calling the specification ready to guide independent implementation, reviewers
must agree operation coverage (section 1), finite-job behavior (section 2), status rules (section 7), retry
lifetimes (section 8), and resource response completeness (section 9). Link separate contracts for
CPU/resource units, archive/config format, auth issuance/browser integration, public
exposure, and log/metric schemas/filtering/streaming/retention (#29/#30).
The 100-replica limit and per-user quotas still require agreement; a per-deployment
limit alone does not limit total user consumption.

Implementation follow-ups include HTTP/query/mutation use cases; adapting client
models; durable acceptance/idempotency storage; domain fields for revision/generation,
workload type, diagnostics, completion and deletion; and worker reporting alignment
through #25. These are not code changes requested by this PR.

Verification should cover cross-user access, keyed replay/conflicts, partial acceptance
failures, every status branch, name/ID preservation, scaling preservation, stop/autoscale,
revision replacement, and successful/failed job completion.

The API layer handles transport, decoding, credential extraction, and response/error
encoding. Applications enforce authorization/lifecycle/acceptance through ports;
domain rules derive state; infrastructure implements persistence, queues, worker
communication and observability adapters. Business rules remain outside HTTP handlers.

## 11. Communication and failure flows

```text
Submit:
  POST /v1/systems + Idempotency-Key + name/code/config
  <- 202 system, Location (durably accepted)
  GET system and deployments until converged, completed, or failed
  If POST times out: retry identical input with SAME key

Scale service:
  PATCH /v1/deployments/{id} {"desiredReplicas": 5}
  <- 202 deployment with new generation
  GET deployment until converged or failure; generation equality alone is insufficient

Inspect failure:
  GET deployment -> status, reason, message
  GET /v1/replicas?deploymentId={id}&revision={currentRevision}
  GET /v1/replicas/{failedId}/logs once observability contract is available
```

| Failure | Observable result |
| --- | --- |
| Invalid archive/config | 422; accepted state unchanged |
| Upload limit exceeded | 413; accepted state unchanged |
| Duplicate system name | 409; no new system |
| Dependency failure before acceptance | 503; no partial client-visible new state; keyed retry resolves uncertain create outcome |
| Capacity unavailable | Accepted work stays PENDING/DEGRADED with INSUFFICIENT_CAPACITY |
| Worker lost | Replicas FAILED; services replaced; jobs not automatically retried under proposed rules |
| Update during rollout | Latest accepted input wins; superseded replicas stopped before replacements |
| Stop cannot be confirmed | Unconverged STOPPING/UPDATING with reason; no unsafe replacement |
| Repeated delete during cleanup | 202; after removal 404 |
| Mutation during deletion | 409 |
| Missing/foreign resource | 404 |
