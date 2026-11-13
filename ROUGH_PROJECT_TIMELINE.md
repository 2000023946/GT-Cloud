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



Yep. I’d make the six sub-issues like this:

### 1. Data Plane — Runtime: Implement & Test `configure()`

**Description:**

> Implement the Runtime `configure()` method.
>
> Prepare a job for execution, including its program, arguments, environment, working directory, and any Runtime-owned state required before starting the process.
>
> Include:
>
> * Functional tests
> * Failure/error tests
> * Concurrency tests where applicable
> * Latency/performance testing
> * Logging and metrics
> * Documentation of the method and its behavior
> * Documentation of tests performed and their results, including latency measurements

---

### 2. Data Plane — Runtime: Implement & Test `start()`

**Description:**

> Implement the Runtime `start()` method to launch a configured job as a Linux process.
>
> The Runtime should create and track the process information needed for later monitoring, stopping, and cleanup.
>
> Include:
>
> * Functional tests
> * Failure/error tests
> * Concurrent process-start tests
> * Latency/performance testing
> * Logging and metrics for process starts and failures
> * Documentation of the method and process state
> * Documentation of tests performed and their results, including latency measurements

---

### 3. Data Plane — Runtime: Implement & Test `monitor()`

**Description:**

> Implement the Runtime `monitor()` method to perform a single observation of a job's current process state.
>
> Return the appropriate `RuntimeState`, such as running, exited, signaled, or not found.
>
> Include:
>
> * Running-process tests
> * Exited-process tests
> * Signaled-process tests
> * Missing/unknown job tests
> * Concurrent monitoring tests
> * Latency/performance testing
> * Logging and metrics for monitoring operations and results
> * Documentation of the method and `RuntimeState` behavior
> * Documentation of tests performed and their results, including latency measurements
>
> `monitor()` should perform one observation and should not contain its own monitoring loop or orchestrate other Runtime operations.

---

### 4. Data Plane — Runtime: Implement & Test `stop()`

**Description:**

> Implement the Runtime `stop()` method to terminate a running job.
>
> The method should correctly handle running processes, already-exited processes, and unknown jobs.
>
> Include:
>
> * Functional tests
> * Failure/error tests
> * Concurrent stop tests
> * Latency/performance testing
> * Logging and metrics for process termination and failures
> * Documentation of termination behavior and error handling
> * Documentation of tests performed and their results, including latency measurements

---

### 5. Data Plane — Runtime: Implement & Test `cleanup()`

**Description:**

> Implement the Runtime `cleanup()` method to remove Runtime-owned state and resources associated with a job.
>
> Cleanup should leave the Runtime in a consistent state and prevent process or Runtime-state leaks.
>
> Include:
>
> * Functional tests
> * Failure/error tests
> * Cleanup after successful execution
> * Cleanup after failed execution
> * Concurrent cleanup tests
> * Process/state leak tests
> * Latency/performance testing
> * Logging and metrics for cleanup operations and failures
> * Documentation explaining what cleanup removes and when it should be used
> * Documentation of tests performed and their results, including latency measurements

---

### 6. Data Plane — Runtime: Lifecycle & Randomized Stress Testing

**Description:**

> Test the Runtime as a complete system across multiple jobs and different operation sequences.
>
> Test:
>
> * Full lifecycle: `configure → start → monitor → stop → cleanup`
> * Invalid or incomplete lifecycle sequences
> * Multiple jobs running concurrently
> * Randomized operation sequences
> * Randomized concurrent operations
> * Runtime state consistency
> * Process leaks
> * Runtime state leaks
> * Long-running stress tests
>
> Record and document:
>
> * Number of tests and operations
> * Successful and failed operations
> * Latency results
> * Failures or leaks found
> * Random seed when applicable
> * Any issues discovered and fixes made
>
> The final Runtime should remain stable and consistent under normal, invalid, concurrent, and randomized workloads.
