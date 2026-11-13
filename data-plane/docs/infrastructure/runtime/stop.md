## StopRuntime – Basic Algorithm

Purpose: Safely terminate a running process associated with a job.

### 1. Algorithm

1. Find process: Look up the job's PID in `RuntimeState`.
2. Send SIGTERM: Request graceful process termination.
3. Wait: Poll `waitpid()` to check whether the process has exited (up to 2 seconds).
4. Force termination: If still running, send SIGKILL.
5. Confirm termination: Wait again for the process to exit.
6. Record telemetry: Log success or failure and update metrics.

### 2. Performance Benchmark

Tested with 100,000 stop operations at different thread counts.

| Threads | Throughput          | Average time/call |
| ------- | ------------------- | ----------------- |
| 1       | 345,742 calls/sec   | 2.89 µs           |
| 4       | 1,192,217 calls/sec | 0.84 µs           |
| 8       | 1,439,247 calls/sec | 0.69 µs           |
| 16      | 1,545,836 calls/sec | 0.65 µs           |

### 3. Key Findings

- Concurrency: Multiple threads can execute stop operations concurrently.
- Scaling: Throughput increases as thread count increases.
- Diminishing returns: Performance gains become smaller at higher thread counts.

Important: The benchmark uses a mock OS adapter that simulates immediate process termination. It measures runtime orchestration, state lookup, synchronization, and telemetry—not actual Linux process termination.

### Conclusion

StopRuntime provides a structured process-termination algorithm with graceful shutdown, forced termination, confirmation, and telemetry. The benchmark demonstrates good concurrent throughput, reaching approximately 1.55 million operations per second with 16 threads.