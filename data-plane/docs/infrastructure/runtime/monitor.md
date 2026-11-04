# Runtime Monitor

## Purpose

The runtime monitor checks whether a job is still running, has exited, or has failed. It reports the runtime state and saves terminal statuses to prevent repeatedly checking completed processes.

## Architecture

```text
Job
 |
 v
Monitor::monitor()
 |
 +--> Log monitoring event
 |
 +--> Increment monitor.checks metric
 |
 +--> Check saved terminal status
 |
 +--> Look up process and PID
 |
 +--> waitpid(PID, WNOHANG)
 |       |
 |       +--> 0  -> Running
 |       +--> PID -> Exited or Signaled
 |       +--> -1 -> Error
 |
 +--> Save terminal status
 |
 v
Return RuntimeStates
```

## Key Design Decisions

- **Non-blocking:** `WNOHANG` prevents the monitor from waiting for a process to finish.
- **State caching:** Exited and signaled statuses are saved to avoid repeated system calls.
- **Thread-safe state:** `RuntimeState` synchronizes shared process and status access.
- **Testability:** An injectable `waitpid` function allows deterministic unit tests and performance benchmarks.
- **Observability:** Each monitoring call logs an event and increments `monitor.checks`.

## Performance Results

Measured locally using a mocked `waitpid`, no-op logging and metrics, and 100,000 calls per thread.

| Threads | Total calls | Total time | Throughput |
|---:|---:|---:|---:|
| 1 | 100,000 | 364.04 ms | 274,697 calls/s |
| 4 | 400,000 | 405.18 ms | 987,226 calls/s |
| 8 | 800,000 | 650.74 ms | 1,229,375 calls/s |
| 16 | 1,600,000 | 1.26 s | 1,266,391 calls/s |

**Result:** Throughput increased from approximately 275K calls/s in the single-thread test to 1.27M calls/s with 16 threads.

*Benchmark note: These are preliminary microbenchmark results, not production process-monitoring measurements. The single-thread and concurrent runs may differ in system load and measurement overhead. Real `waitpid` behavior and production observability have not been benchmarked.*

## Testing

- Missing process and PID
- Running, exited, and signaled processes
- `waitpid` errors and unexpected results
- Cached terminal statuses
- Logging and metrics
- Concurrent monitoring with 4, 8, and 16 threads
