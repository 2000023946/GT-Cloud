# RuntimeState

## Overview

`RuntimeState` is the in-memory store used by the GT Cloud data plane to track process metadata and retain child-process stack allocations.

It uses two independently synchronized hash maps:

```rust
pub struct RuntimeState {
    processes: RwLock<HashMap<String, ProcessState>>,
    stacks: RwLock<HashMap<String, Vec<u8>>>,
}
```

* `processes` maps job IDs to their process metadata.
* `stacks` maps job IDs to stack allocations.
* `RwLock` protects concurrent access to each map.

## ProcessState

Each process entry contains:

| Field               | Description                           |
| ------------------- | ------------------------------------- |
| `program`           | Executable to launch.                 |
| `args`              | Command-line arguments.               |
| `working_directory` | Optional working directory.           |
| `environment`       | Explicit environment variables.       |
| `pid`               | Optional operating-system process ID. |

## API Operations

| Method               | Responsibility                                                               |
| -------------------- | ---------------------------------------------------------------------------- |
| `new()`              | Creates an empty state.                                                      |
| `add_process()`      | Inserts or replaces a process.                                               |
| `get_process()`      | Retrieves a process by job ID.                                               |
| `set_pid()`          | Updates a process PID or returns an error if the job is missing.             |
| `contains_process()` | Checks whether a process exists.                                             |
| `remove_process()`   | Removes and returns a process.                                               |
| `process_count()`    | Returns the number of stored processes.                                      |
| `add_stack()`        | Inserts or replaces a stack allocation.                                      |
| `with_stack()`       | Accesses a stack through a closure while holding the stack map's write lock. |
| `contains_stack()`   | Checks whether a stack exists.                                               |
| `remove_stack()`     | Removes and returns a stack allocation.                                      |
| `stack_count()`      | Returns the number of stored stacks.                                         |

## Thread Safety

Each map has its own `RwLock`, protecting its operations against unsynchronized concurrent access.

The process map and stack map are independent. Removing a process does not automatically remove its stack, and removing a stack does not automatically remove its process.

`with_stack()` holds the stack map's write lock while its closure executes. This can serialize access to different stack entries during that closure.

Operations across both maps are not automatically atomic. The runtime must coordinate process removal and stack cleanup with the actual child-process lifecycle.

## Lifecycle

1. `StartRuntime` allocates a child stack.
2. The child process is created.
3. The runtime records the process and PID.
4. The stack remains retained while the child may need it.
5. After the child has terminated and cleanup is safe, the runtime removes the process and stack entries.

## Performance Benchmarks

### Objective

The benchmarks measure in-memory RuntimeState throughput for sequential and concurrent process operations.

Tested state sizes were 1,000, 10,000, and 100,000 jobs. Concurrent tests used 1, 2, 4, 8, and 16 threads.

Operations included insert, get, contains, PID update, remove, and a mixed workload of 80% gets, 10% existence checks, and 10% PID updates.

### Sequential Results

All values are operations per second.

|    Jobs |    Insert |       Get |  Contains | Update PID |    Remove |
| ------: | --------: | --------: | --------: | ---------: | --------: |
|   1,000 | 1,485,000 | 1,507,000 | 6,065,000 |  5,988,000 | 2,524,000 |
|  10,000 | 1,928,000 | 1,510,000 | 7,919,000 |  8,717,000 | 3,783,000 |
| 100,000 | 2,121,000 | 1,805,000 | 5,549,000 |  5,217,000 | 2,791,000 |

### Concurrent Results at 100,000 Jobs

All values are operations per second.

| Threads |       Get |  Contains | Update PID |     Mixed |    Insert |    Remove |
| ------: | --------: | --------: | ---------: | --------: | --------: | --------: |
|       1 | 1,739,790 | 4,613,176 |  4,613,539 | 1,991,967 | 1,558,524 | 1,953,411 |
|       2 | 2,272,968 | 6,512,272 |  4,498,889 | 2,504,558 | 2,975,191 | 3,416,618 |
|       4 | 5,040,407 | 5,518,573 |  3,812,901 | 2,773,704 | 2,991,105 | 2,423,909 |
|       8 | 3,113,018 | 2,661,751 |  2,071,834 | 2,204,280 | 1,835,387 | 1,501,505 |
|      16 | 3,210,548 | 1,305,651 |  2,163,683 | 2,027,702 | 2,097,809 | 1,309,223 |

### Findings

* The highest recorded mixed-workload throughput was approximately **2.77 million operations/sec at four threads**.
* The highest recorded `get` throughput was approximately **5.04 million operations/sec at four threads**.
* The highest recorded `contains` throughput was approximately **6.51 million operations/sec at two threads**.
* Increasing the thread count did not consistently increase throughput.

Lock contention and thread scheduling overhead are possible explanations for the non-monotonic results; the benchmark does not establish the cause.

### Limitations

These results are initial measurements from a single recorded run. Concurrent timing includes thread creation and joining overhead, and the total operation count is divided among the selected threads.

The original machine and build configuration were not recorded alongside the measurements. The results should therefore be treated as a baseline, not a production capacity guarantee.

Future benchmarks should include warm-up runs, repeated measurements, medians and variability, worker-thread reuse, and recorded hardware and software configuration.

These tests measure in-memory state operations, not end-to-end workload startup or execution.

## Conclusion

`RuntimeState` provides the in-memory process and stack tracking used by the data plane. The initial benchmark indicates that throughput depends on the operation type and thread count. More repeatable measurements are needed before changing the synchronization strategy or making production performance claims.
