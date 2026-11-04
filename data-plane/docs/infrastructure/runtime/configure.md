### Runtime `configure()`

The `configure()` method prepares a job for execution without starting the process.

It takes a `Job` and extracts the information needed by the Runtime:

* **Program** — executable to run.
* **Arguments** — arguments passed to the program.
* **Environment** — environment variables for the process.
* **Working directory** — generated from the job ID using `WorkingDirectoryManager`.
* **Runtime state** — stores the resulting `ProcessState` in `RuntimeState`.

`configure()` does **not** start or manage the process. Process execution is handled by the Runtime `start()` operation.

### Testing

Functional tests verify that:

* Program and arguments are extracted correctly.
* Environment variables are stored correctly.
* The correct working directory is generated.
* Multiple jobs can be configured.
* Existing job state is replaced correctly.

Failure tests verify that invalid commands, including empty or whitespace-only commands, return an error instead of creating invalid process state.

### Performance

The configuration process was tested with:

* 1,000 jobs
* 10,000 jobs
* 100,000 jobs

The test measures total configuration time, average latency per job, and throughput while verifying that all jobs were successfully stored in `RuntimeState`.

Example results from the test run:

|    Jobs | Total Time | Avg. Time/Job |       Throughput |
| ------: | ---------: | ------------: | ---------------: |
|   1,000 |    4.44 ms |       4.44 µs | 224,974 jobs/sec |
|  10,000 |   38.06 ms |       3.81 µs | 262,745 jobs/sec |
| 100,000 |  385.85 ms |       3.86 µs | 259,165 jobs/sec |

### Observability

The `configure()` operation is intended to record successful and failed configuration events and its execution latency through Runtime logging and metrics.
