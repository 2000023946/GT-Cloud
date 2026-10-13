# Start Runtime

## Purpose

`StartRuntime` starts a configured job as a Linux process. It uses the
process information prepared by `ConfigureRuntime` and records the
started process in `RuntimeState`.

Process isolation is Linux-specific; the runtime tests should run in a
Linux environment.

## High-level flow

1.  **Receive the job** --- `StartRuntime::start` receives the runtime
    state and job.
2.  **Find configured process state** --- look up the job by ID. If the
    job has not been configured, startup fails.
3.  **Create the child process** --- the process-creation helper starts
    a child using Linux `clone` and the configured namespace flags.
4.  **Prepare execution** --- the child-process helper sets the working
    directory, environment, and stdout/stderr files, then executes the
    configured program and arguments.
5.  **Record the PID** --- the runtime stores the child PID in the job's
    process state.
6.  **Return the result** --- startup returns success or an error. A
    successful return means the startup operation succeeded; it does not
    necessarily mean the application is ready to serve requests.

## Helpers

-   **`create_process`** --- creates the isolated child process and
    invokes the child callback.
-   **`child_process`** --- configures the child process environment and
    redirects output to `stdout.txt` and `stderr.txt` in the working
    directory before executing the requested program.
-   **`RuntimeState`** --- stores per-job process information, including
    the configured program, arguments, working directory, and PID.
-   **`ConfigureRuntime`** --- prepares the job's executable, arguments,
    and working directory before `StartRuntime` is called.

## Tests

The runtime tests cover configuration, successful process startup, PID
namespace isolation, rejection of unconfigured jobs, and running a
Python script while checking captured output.

The performance test runs two types of measurements at concurrency
levels 1, 4, 8, and 16:

-   **Burst startup:** starts a small batch concurrently and measures
    startup latency and batch throughput.
-   **Sustained-load loop:** workers repeatedly start short-lived
    `/bin/true` jobs, recording latency, successful starts, failures,
    and throughput.

The benchmark times the `StartRuntime::start` call for individual
startup latency. Throughput is calculated from successful starts divided
by elapsed batch time. The current sustained-load test has a cap of 500
attempts per concurrency level; the reported runs reached that cap in
less than one second. Therefore, those results measure a capped batch
rather than a full 10-second sustained load.

## Latest benchmark snapshot

Results from the reported Docker/Linux run:

  --------------------------------------------------------------------------
      Concurrent     Successful        Average    p95 startup       Reported
         workers         starts        startup                    throughput
  -------------- -------------- -------------- -------------- --------------
               1            500       0.483 ms       0.806 ms         864.77
                                                                  starts/sec

               4            500       0.814 ms       1.137 ms       2,206.70
                                                                  starts/sec

               8            500       1.883 ms       3.084 ms       2,339.67
                                                                  starts/sec

              16            500       4.045 ms       7.401 ms       2,078.95
                                                                  starts/sec
  --------------------------------------------------------------------------

These are preliminary results from the specific Docker/Linux
environment, not a universal capacity limit. The sustained-load results
are capped at 500 attempts, and throughput includes scheduling and
cleanup effects. Run repeated tests with a larger attempt cap or a
genuinely fixed-duration workload before drawing conclusions about
maximum sustained throughput.

## Important limitations

-   Startup latency measures how long `StartRuntime::start` takes to
    return, not time until the program finishes or becomes ready.
-   Process creation and namespace setup are part of the startup cost.
-   The benchmark uses `/bin/true` to keep the workload short; it does
    not measure application execution performance.
-   Performance depends on the Linux kernel, Docker configuration, host
    resources, and concurrent workload.
