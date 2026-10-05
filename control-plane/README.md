# Control Plane

The control plane maintains desired state and reconciles it with current state.

```text
User / Async Events
        ↓
Reconciliation Queue
        ↓
Deployment Reconciler
        ↓
Deployment Controller
        ↓
Actions
        ↓
Action Executor
   ┌────┴────┐
 Create     Stop
   ↓          ↓
Scheduler   ReplicaRepository
   ↓          ↓
WorkerClient ← Worker
        ↓
    Data Plane
```

The control plane also maintains the platform's global view of running replicas and their network endpoints.

```text
Data Plane
    │
    │ Endpoint
    ▼
Endpoint Registry
    │
    ▼
Endpoint Repository
    ▲
    │
Endpoint Discovery
    │
    ▼
Endpoint Selector
    │
    ▼
DNS Service
```

Persistence stores platform state and artifacts. The reconciliation queue is the fan-in point for user changes, worker failures, autoscaling events, and other state changes.

* **API** — accepts user requests.
* **Persistence** — stores platform state and artifacts.
* **Scheduling/Reconciliation** — converges actual state toward desired state.
* **Worker** — communicates with the data plane.
* **Service Discovery** — maintains and resolves service endpoints.
* **DNS** — exposes service discovery to workloads through normal DNS resolution.

## Structure

```text
control-plane/

├── api/             # API layer
├── applications/    # Use cases / application logic
├── domain/          # Core domain models and rules
├── ports/           # Interfaces between layers
├── docs/            # Architecture diagrams and design documentation
├── bootstrap/       # Application wiring
└── cmd/main/        # Application entry point
```

### Layers

* **`api/`** — Handles external API requests.

* **`applications/`** — Contains use cases and application logic, including reconciliation, endpoint registration, and service discovery.

* **`domain/`** — Contains core platform objects and business rules.

* **`ports/`** — Defines interfaces that application logic uses to communicate with persistence, workers, and other infrastructure.

* **`bootstrap/`** — Wires implementations and dependencies together.

* **`cmd/main/`** — Starts the control-plane application.

* **`docs/`** — Contains architecture diagrams and design documentation.

The control plane follows a dependency-inward design: **application logic depends on interfaces (`ports`), while infrastructure provides their implementations.**
