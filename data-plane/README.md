# Data Plane

The data plane runs replicas on workers and manages their execution, resources, and networking.

```text
Control Plane
      ↓
 Worker Client
      ↓
 Data Plane API
      ↓
 Applications
      ↓
 Execution
   ┌──┼──────────┐
   ↓  ↓          ↓
Runtime Resources Networking
          ↓          ↓
      Resource    Network
       Manager     Manager
```

The data plane is responsible for executing work assigned to it by the control plane.

```text
Start
  ↓
Prepare
  ├── Resources
  ├── Networking
  └── Runtime
  ↓
Run
  ↓
Monitor
  ├── Resources
  ├── Networking
  └── Runtime
  ↓
Stop
  ↓
Cleanup
```

The data plane communicates its current state and runtime information back to the control plane.

* **API** — Handles communication between the control plane and data plane.
* **Execution / Runtime** — Coordinates the replica lifecycle and manages the execution environment and user processes.
* **Resources** — Configures and monitors CPU and memory resources.
* **Networking** — Configures and monitors the replica's network environment and connectivity.

Service discovery and DNS are handled by the control plane. The data plane configures the network environment required for replicas to communicate through the control plane's service discovery system.

## Structure

```text
data-plane/

├── docs/                  # Architecture and design documentation
├── src/
│   ├── api/               # Data-plane API boundary
│   ├── applications/      # Data-plane use cases
│   ├── domain/            # Core data-plane models and rules
│   ├── infrastructure/    # Infrastructure implementations
│   ├── ports/             # Interfaces
│   ├── bootstrap/         # Dependency wiring
│   └── main.rs            # Application entry point
├── Cargo.toml             # Rust project configuration
└── Cargo.lock             # Locked dependency versions
```

The data plane follows the same dependency-inward design as the control plane: **application logic depends on interfaces (`ports`), while infrastructure provides their implementations.**
