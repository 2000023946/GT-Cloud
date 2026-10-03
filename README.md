
# GT Cloud

GT Cloud is a student-built infrastructure platform that allows users to
submit applications and run them on a pool of machines.

The project is being built from scratch to learn and implement the core
systems involved in a cloud execution platform.

## V1

V1 allows a user to:

- Submit an application
- Specify CPU, memory, and network requirements
- Run the application on an available worker
- View job status
- Stop a running job

The platform consists of three main components:

```text
User
  |
  v
Client
  |
  | HTTP
  v
Control Plane
  |
  | Worker protocol
  v
Data Plane
  |
  v
Linux
```

## Repository Structure

```text
gt-cloud/
├── client/          # CLI and Web client
├── control-plane/   # Scheduling and platform control
├── data-plane/      # Job execution on workers
└── V1.md            # V1 requirements
```

### Client

The client provides the user-facing interface.

```text
CLI / Web
    |
    v
Client API
    |
    v
Control Plane
```

CLI and Web share the same client application logic.

### Control Plane

The control plane manages the platform.

Responsibilities include:

* Job management
* Worker management
* Scheduling
* Worker monitoring
* Job monitoring
* Self-healing
* Communication with workers

### Data Plane

The data plane runs on worker machines.

Responsibilities include:

* Running jobs
* Managing CPU and memory resources
* Configuring networking
* Process isolation
* Reporting job status
* Sending worker heartbeats

## Architecture

The project uses a ports-and-adapters architecture.

```text
Domain
  |
Ports
  |
Application
  |
Infrastructure
  |
Bootstrap
  |
API
```

The core application logic depends on interfaces/contracts rather than
concrete infrastructure implementations.

This allows components to be developed and tested independently.

## Development

The project is currently in the architecture and implementation phase.

The initial implementations are intentionally minimal. Concrete
infrastructure will progressively replace the current skeletons.

## Goal

Build a small cloud-like execution platform from the ground up while
learning the systems involved in scheduling, process execution, resource
management, networking, monitoring, and distributed systems.
