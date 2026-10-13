# Stop Runtime

## Overview

`StopRuntime` terminates a running job by sending `SIGTERM` to its process. It uses `RuntimeState` to retrieve the job's PID and a `StopRuntimeOsAdapter` to isolate OS-level process signaling.

## Execution Flow

1. **Retrieve PID:** Look up the job's process in `RuntimeState`. Return an error if the process or PID is missing or invalid.
2. **Send SIGTERM:** Call `os_adapter.kill(pid, SIGTERM)` to request graceful process termination.
3. **Handle result:** On success, record success telemetry. On failure, retrieve the OS error and map it to a descriptive message (e.g., process not found or permission denied).
4. **Record telemetry:** Log the outcome and increment the corresponding success or failure metric.

## Concurrency

`StopRuntime` uses shared references and does not maintain mutable per-call state. `RuntimeState` protects its stored data with synchronization primitives, while the OS adapter, logger, and metrics implementations must support concurrent access for the service to be safely shared across threads.

Concurrent calls to stop the same job may send multiple termination signals; the operation is not atomic across PID lookup and signaling.

## Performance Test

The performance test executes 100,000 mocked stop operations at each concurrency level. The mock adapter simulates successful process signaling, so results measure application logic and telemetry overhead—not actual OS process termination.

| Threads | Elapsed time | Throughput | Average per call |
|---:|---:|---:|---:|
| 1 | 0.300 s | 333,508 calls/s | 3.00 µs |
| 4 | 0.085 s | 1,176,272 calls/s | 0.85 µs |
| 8 | 0.065 s | 1,542,015 calls/s | 0.65 µs |
| 16 | 0.064 s | 1,550,797 calls/s | 0.64 µs |

Throughput improved with concurrency and plateaued between 8 and 16 threads. These results are a single benchmark run and should be treated as indicative rather than definitive.

**Run the test:**

```bash
cargo test performance_stop_runtime_100k_calls_at_1_4_8_16_threads -- --nocapture
```