# Exercise 33: Tokio `watch` for graceful worker shutdown

## Theory

`tokio::sync::watch` stores one latest value and allows many receivers to
observe it. This makes it useful for configuration updates and broadcast
shutdown signals.

Create a channel with an initial value:

```rust
let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
```

The sender publishes a new value:

```rust
shutdown_tx.send(true).unwrap();
```

Every receiver can observe the update independently. A worker usually waits
for either a new job or a shutdown update:

```rust
tokio::select! {
    job = job_rx.recv() => {
        // process the job
    }
    changed = shutdown.changed() => {
        changed?;
        if *shutdown.borrow() {
            // finish cleanup and stop
        }
    }
}
```

Unlike `oneshot`, a `watch::Receiver` can be cloned for every worker. All
workers observe the same latest state, and a worker that subscribes after the
update can still see `true` by reading `borrow()`.

`watch` does not queue every historical value. If the sender publishes `true`
and then `false`, a slow receiver may only observe the latest state. That is
appropriate for state such as "shutdown requested", but not for a sequence of
individual jobs; use `mpsc` for jobs.

Graceful shutdown means stopping new work, allowing the current small
operation to finish, cleaning up resources, and then returning. `abort()` is a
hard stop and should be reserved for tasks that do not finish in time.

## Task

Implement `process_until_shutdown` in `main.rs`:

- Reject `worker_count == 0` with `InvalidWorkerCount`.
- Create a bounded job channel for each worker and one result channel.
- Clone the supplied `watch::Receiver<bool>` for every worker.
- Spawn exactly `worker_count` workers.
- Each worker must use `tokio::select!` to wait for either a job or a shutdown
  update.
- On a shutdown value of `true`, stop accepting jobs and exit the worker loop
  cleanly.
- Have the coordinator use `select!` while dispatching and collecting so it
  returns `ShutdownRequested` when shutdown is observed.
- Preserve input ordering using `(index, value)` jobs and indexed results.
- Await every worker handle and return `WorkerPanicked` if one panics.

Do not use `Arc<Mutex<_>>` for coordination. Use `mpsc` channels for jobs and
results, and `watch` only for the shared shutdown state.

## Expected behavior

- With shutdown set to `false`, `[1, 2, 3, 4]` returns `[2, 4, 6, 8]`.
- If shutdown is already `true`, return `ShutdownRequested`.
- Zero workers returns `InvalidWorkerCount`.
- Workers must finish or shut down before the function returns.

## Run

```bash
make 02_concurrency_12_tokio_watch_shutdown
```

## Follow-up

Explain why `watch` is suitable for a broadcast state change, why `mpsc` is
still needed for jobs, how `borrow()` differs from `changed().await`, and how
graceful shutdown differs from `JoinHandle::abort()`.
