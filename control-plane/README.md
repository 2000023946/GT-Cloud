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

Persistence stores platform state. The queue is the fan-in point for user changes, worker failures, and autoscaling events.

* **API** — accepts user requests.
* **Persistence** — stores state and artifacts.
* **Scheduling/Reconciliation** — converges desired state to current state.
* **Worker** — communicates with the data plane.

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
* **`applications/`** — Contains use cases, broken into application interfaces and their services.
* **`domain/`** — Contains core platform objects and business rules.
* **`ports/`** — Defines interfaces used by applications to communicate with infrastructure and other layers.
* **`bootstrap/`** — Wires implementations and dependencies together.
* **`cmd/main/`** — Starts the control-plane application.
* **`docs/`** — Contains architecture diagrams and design documentation.

The control plane follows a dependency-inward design: application logic depends on interfaces (`ports`), while infrastructure provides their implementations.
