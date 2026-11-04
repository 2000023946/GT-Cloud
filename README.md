# AWS

A distributed cloud execution platform built from scratch.

The repository is organized into three primary components:

```text
aws/

├── client/          # User-facing interfaces
├── control-plane/   # Platform control and orchestration
├── data-plane/      # Workload execution on workers
└── version/         # Version specifications and planned features
```

### Components

* **`client/`** — Provides the user-facing CLI and web interfaces for interacting with the platform.

* **`control-plane/`** — Manages desired state, scheduling, reconciliation, persistence, workers, and service discovery.

* **`data-plane/`** — Runs replicas on workers and manages their execution, resources, and networking.

* **`version/`** — Contains version-specific architecture, requirements, and planned features.

### Documentation

Each component contains its own `README.md` describing its architecture and structure.

Additional architecture diagrams and design documentation are available in the `docs/` directories within each component.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for contribution guidelines.
