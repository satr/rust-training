# Exercise 29: Tokio `mpsc` channels

## Theory

Tokio's multi-producer, single-consumer (`mpsc`) channel lets several tasks
send owned values to one receiver:

```rust
let (tx, mut rx) = tokio::sync::mpsc::channel(8);
let worker_tx = tx.clone();

tokio::spawn(async move {
    worker_tx.send(42).await.unwrap();
});

drop(tx);
while let Some(value) = rx.recv().await {
    println!("{value}");
}
```

`channel(8)` is bounded: at most eight messages wait in the channel. When it
is full, `send().await` yields until the receiver makes room. This is
backpressure, which prevents an unlimited producer from growing memory
without bound. The channel is closed after the receiver is dropped or after
every `Sender` has been dropped; `recv().await` then returns `None`.

Each cloned `Sender` is owned by one producer task. The original sender must
also be dropped when the parent will no longer send, otherwise the receiver
will keep waiting for a sender that can never produce another value.

This is similar to `System.Threading.Channels.Channel<T>` in C# and a Go
channel. Rust's channel API makes the ownership of each sender explicit. A
bounded channel is also a useful backend pattern for limiting queued work.

Do not await all producers before receiving from a bounded channel: if the
workers produce more messages than the channel capacity, they can block on
`send().await` while the parent is blocked waiting for those same workers.
Receive concurrently, then await the task handles afterward if their failures
must be observed.

## Task

Implement `collect_async_worker_values` in `main.rs`:

- Create a bounded Tokio `mpsc` channel.
- Spawn `worker_count` tasks.
- Give each task its own cloned sender.
- Each task sends its worker index `values_per_worker` times.
- Drop the parent's original sender after all producer tasks are spawned.
- Receive values until the channel closes and return them in a `Vec<usize>`.
- Use `unwrap()` for send failures in this small exercise.

Use a channel and tasks; do not use `Arc`, `Mutex`, shared mutable state, or
`std::thread`.

## Expected behavior

Three workers producing two values each yield two copies of every worker
index. Message order is not guaranteed, so callers may sort the result:

```text
[0, 0, 1, 1, 2, 2]
```

Zero workers and workers configured to send zero values return an empty vector.

## Run

```bash
make 02_concurrency_08_tokio_channels
```

## Follow-up

Explain why the sender is cloned, why the original sender must be dropped,
what backpressure means, and why receiving must happen before awaiting every
producer when the channel is bounded.
