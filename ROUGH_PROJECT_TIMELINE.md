# GT Cloud Platform — Rough Project Plan

## 1. Architecture

The architecture is defined around three major components:

```text
Client
   ↓
Control Plane
   ↓
Data Plane
```

* **Client** — User-facing interfaces.
* **Control Plane** — Platform control and orchestration.
* **Data Plane** — Workload execution, resources, and networking.

The high-level architecture is now set, so implementation can begin.

---

## 2. October — Build the Components

Everyone chooses one area to work on.

* First come, first served.
* Maximum 3 people per area.
* Each group owns its area through October.
* Each group creates its own GitHub issues and divides the work.
* See **CONTRIBUTING.md** for coding standards, testing requirements, and expectations for completed work.

### Areas

**Client**

1. API / Application / Ports
2. CLI
3. Web

**Control Plane**
4. API / Application
5. Persistence
6. Service Discovery / DNS
7. Worker Client
8. Scheduling / Reconciliation
9. Execution

**Data Plane**
10. API / Application
11. Runtime
12. Networking
13. Resources

### October goal

By around the **first week of November**, the individual areas should be finished according to the standards and Definition of Done in **CONTRIBUTING.md**.

Each group is also responsible for creating tests for the part they implemented.

---

## 3. November — Independent System Testing

Test the **Control Plane and Data Plane separately**.

Each group is responsible for testing the components they built as part of the larger plane.

The focus should be on **designing tests that expose problems**, not just tests that prove the code works.

Test to:

* Find bugs
* Test failures and recovery
* Find bottlenecks
* Stress the system
* Discover its limits

Fix the problems found and retest.

---

## 4. December — Full-System Testing

Bring the Client, Control Plane, and Data Plane together and test the **full system end-to-end**.

The groups are responsible for helping design and run tests around the parts they own within the full system.

Again, focus on **tests designed to find bugs and expose the limits of the system**, including:

* Stress testing
* Failure and recovery
* Concurrency
* Heavy workloads
* System limits

---

## 5. Spring 2027 — Build Cloud Services

Once the foundation is working, shift toward deploying the platform and building services on top of it.

Potential services include:

* Lambda / serverless
* Databases
* Storage
* Queues
* Load balancing
* Additional cloud services

---

## Rough Timeline

```text
October
→ Build individual areas + tests

~First week of November
→ Areas finished

November
→ Test Control Plane + Data Plane independently
→ Find bugs and system limits

December
→ Full-system integration + heavy testing
→ Find bugs and system limits

Spring 2027
→ Deploy platform + build cloud services
```

**Overall:**

**Fall = Build and prove the platform.**

**Spring = Deploy it and build services on top of it.**
