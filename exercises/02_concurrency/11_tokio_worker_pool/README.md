# Exercise 32: Tokio worker pool with `mpsc` and `select!`

## Theory

Tokio's multi-producer, single-consumer (`mpsc`) channel has many possible
senders but exactly one receiver. Therefore, several workers cannot each call
`recv()` on the same Tokio `mpsc::Receiver`. In this exercise, a coordinator
owns the input jobs and distributes them to workers through one bounded
channel per worker. Each worker owns its channel's receiver and processes jobs
concurrently. Workers send results back through a shared result sender, using
`Sender::clone()` for each worker.

A bounded channel provides backpressure: when a worker's queue is full, the
coordinator waits instead of creating unlimited in-memory work. This is useful
for protecting downstream services and controlling memory usage.

The result of a worker may arrive in a different order from the input. To
return results in input order, attach an index to every job and store each
completed result at that index:

```rust
let (index, value) = job;
let result = value * 2;
results[index] = result;
```

Use `tokio::select!` when a coordinator must react to either normal progress
or cancellation:

```rust
tokio::select! {
    send_result = jobs.send(job) => send_result?,
    _ = &mut cancellation => return Err(WorkerPoolError::Cancelled),
}
```

The losing branch is dropped. If cancellation wins, stop dispatching new jobs,
close the job channel, and ensure spawned workers are awaited or aborted. A
`JoinHandle` must be checked because a worker can panic independently of its
parent task.

This resembles a Go worker pool using channels and `context.Context`, or a C#
producer-consumer pipeline with a cancellation token. Rust makes channel
ownership and task lifetimes explicit.

## Task

Implement `process_with_workers` in `main.rs`:

- Reject `worker_count == 0` with `InvalidWorkerCount`.
- Create one bounded `mpsc` job channel per worker and one result channel.
- Spawn exactly `worker_count` Tokio worker tasks.
- Have the coordinator distribute each indexed job to a worker channel. Each
  worker should multiply the value by two and send the indexed result back.
- Use `tokio::select!` while dispatching jobs so cancellation stops new work
  and returns `WorkerPoolError::Cancelled`.
- Drop all job senders after dispatching all work so workers can finish
  naturally.
- Collect all results and reconstruct the original input order.
- Convert worker panics into `WorkerPoolError::WorkerPanicked`.

Do not use `Arc<Mutex<_>>` for result collection. Use channels and indexed
results instead.

## Expected behavior

- `vec![1, 2, 3, 4, 5]` with two workers returns
  `[2, 4, 6, 8, 10]` in that order.
- A cancellation signal returns `WorkerPoolError::Cancelled`.
- Zero workers returns `WorkerPoolError::InvalidWorkerCount`.
- Worker tasks must not be leaked after the function returns.

## Run

```bash
make 02_concurrency_11_tokio_worker_pool
```

## Follow-up

Explain why worker completion order is nondeterministic, how indexed results
restore input order, why bounded channels create backpressure, and what
happens to the losing branch of `select!`.

### How `process_with_workers` works

The function has four phases: create the communication channels, spawn the
workers, dispatch the jobs, and collect the ordered results.

First, it rejects an unusable worker count and records the number of input
values. It creates one result channel for all workers and stores the job
senders and task handles in vectors:

```rust
let (result_tx, mut result_rx) = mpsc::channel(value_count.max(1));
let mut job_senders = Vec::with_capacity(worker_count);
let mut handles = Vec::with_capacity(worker_count);
```

Each worker gets its own bounded job channel. The coordinator stores the
sender, while the worker owns the receiver. Every worker also receives a clone
of the result sender, so all workers can send results to the same coordinator:

```rust
let (job_tx, mut job_rx) = mpsc::channel(1);
let worker_result_tx = result_tx.clone();
```

A worker waits for `(index, value)`, doubles the value, and sends back
`(index, value * 2)`. The index belongs to the job, not the worker. Workers can
finish in any order, so the index is needed to restore the original input
order later.

The coordinator distributes jobs round-robin:

```rust
let worker = index % worker_count;
```

It uses `select!` to race sending the next job against cancellation. A full
bounded job channel makes the send wait, providing backpressure. The
`&mut cancellation` receiver can be polled repeatedly, and `biased;` gives the
cancellation branch priority when both branches are ready.

After dispatching, the coordinator drops all job senders. This tells workers
that no more jobs are coming. Workers then finish naturally when their
receivers return `None`.

Results may arrive out of order, so the coordinator allocates a result vector
and writes each result at its original index:

```rust
results[index] = value;
```

The coordinator also checks cancellation while collecting results. If
cancellation wins, it stops waiting for more results, aborts the workers, and
awaits their handles. If a worker task panics, awaiting its `JoinHandle`
returns an error, which is converted to `WorkerPanicked`.

### Cleanup and extracting repeated logic

After the coordinator finishes dispatching jobs, it can close all job channels
before checking whether cancellation happened:

```rust
drop(job_senders);

if cancelled {
    abort_workers(handles).await;
    return Err(WorkerPoolError::Cancelled);
}
```

This is safe because the dispatch loop is finished and no more jobs will be
sent. Dropping the senders tells workers that no additional jobs are coming.
In the normal case, workers receive `None` from `recv()` and finish naturally.
In the cancellation case, closing the channels is still useful because it
stops new work from entering the pool before the tasks are aborted.

The repeated cancellation cleanup can be extracted into an async helper:

```rust
async fn abort_workers(handles: Vec<tokio::task::JoinHandle<()>>) {
    for handle in &handles {
        handle.abort();
    }

    for handle in handles {
        let _ = handle.await;
    }
}
```

The helper first calls `abort()` on every task, then awaits every
`JoinHandle`. Awaiting the handles is important because it confirms that the
tasks have stopped and prevents task cleanup from being left unfinished.
Passing `handles` by value is appropriate because the helper consumes the
handles while it waits for them. Both cancellation paths can then reuse the
same helper:

```rust
if cancelled {
    abort_workers(handles).await;
    return Err(WorkerPoolError::Cancelled);
}

if cancelled_while_collecting {
    abort_workers(handles).await;
    return Err(WorkerPoolError::Cancelled);
}
```
