# Client

The client provides user-facing interfaces for interacting with the control plane.

```text
User
 ↓
CLI / Web
 ↓
Application
 ↓
Control Plane API
```

The client is responsible for providing interfaces through which users submit and manage workloads.

* **API / Application / Ports** — Defines the client API boundary, use cases, and interfaces for communication.
* **CLI** — Provides the command-line interface.
* **Web** — Provides the web interface.

## Structure

```text
client/

├── docs/                  # Documentation
├── src/
│   ├── api/               # API boundary
│   ├── application/       # Client use cases
│   ├── domain/            # Core client models
│   ├── infrastructure/    # External integrations
│   ├── ports/             # Interfaces
│   ├── cli/               # Command-line interface
│   ├── web/               # Web interface
│   └── bootstrap/         # Dependency wiring
├── package.json
└── tsconfig.json
```

The client follows the same dependency-inward design: **application logic depends on interfaces (`ports`), while infrastructure provides their implementations.**
