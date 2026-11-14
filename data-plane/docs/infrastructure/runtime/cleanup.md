# GT-Cloud Runtime Cleanup

## How It Works

The `CleanupRuntime` component removes runtime resources after a process has been stopped.

1. **Remove Process:** Removes the process information associated with the job.
2. **Remove Stack:** Removes the job's allocated stack memory.
3. **Telemetry:** Logs whether cleanup succeeded or failed and records the corresponding metric.

The cleanup component does not terminate processes. That responsibility belongs to `StopRuntime`.

## Performance Results

| Threads | Throughput (cleanups/sec) | Average Time |
|---:|---:|---:|
| 1 | 240,451 | 4.159 µs |
| 4 | 472,414 | 2.117 µs |
| 8 | 336,174 | 2.975 µs |
| 16 | 375,321 | 2.664 µs |

## Results

- All 4 tests passed.
- Successfully tested concurrent cleanup of different jobs.
- Successfully tested 20 threads cleaning up the same job.
- Best measured performance was **472,414 cleanups per second using 4 threads**.

**Conclusion:** Runtime cleanup successfully removes process and stack resources, handles concurrent cleanup operations, and achieves its highest measured throughput with four threads.