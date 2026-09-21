# Exercise 26: Threads and channels

## Theory

Message passing lets threads communicate by sending values through a channel
instead of sharing mutable state. Rust's standard multiple-producer,
single-consumer channel is created with:

```rust
let (sender, receiver) = std::sync::mpsc::channel();
```

`sender.send(value)` sends a value. `receiver.recv()` waits for one value, and
the receiver can be iterated to receive values until every sender is dropped:

```rust
for value in receiver {
    values.push(value);
}
```

For example:

```rust
sender.send(worker_id).unwrap();
```

`send` takes ownership of `worker_id` and returns a `Result<(), SendError<T>>`.
For a `Copy` value such as `usize`, this ownership transfer is inexpensive. A
successful send returns `Ok(())`. If all receivers have been dropped, sending
fails and the error contains the value that could not be delivered. `.unwrap()`
is acceptable in this small exercise because a missing receiver indicates a
programming/setup failure; production code should usually handle the error or
propagate it.

For production code, handle the failure explicitly when sending is best-effort:

```rust
if let Err(error) = worker_sender.send(worker_id) {
    eprintln!("worker could not send value: {error}");
}
```

Or propagate the error from a function that returns a compatible `Result`:

```rust
worker_sender
    .send(worker_id)
    .map_err(|error| format!("failed to send value: {error}"))?;
```

The sender is usually moved into a spawned thread. To create multiple
producers, clone the sender with `sender.clone()`. The channel closes when the
last sender is dropped, allowing the receiver loop to finish.

The original sender remains alive after cloning it, so explicitly drop it after
all worker senders have been created:

```rust
drop(sender);
```

Each worker owns its sender because it is moved into the `thread::spawn`
closure. That sender is automatically dropped when the worker finishes. The
receiver loop can finish only after the original sender is dropped and every
worker sender has been dropped. If the original sender remains alive, the
receiver may wait forever for another value.

## Task

Implement `collect_worker_values`:

- Create an `mpsc` channel.
- Spawn `worker_count` threads.
- Each worker sends its worker index `values_per_worker` times.
- Clone and move a sender into each worker.
- Drop the original sender before receiving values.
- Collect all received values into a `Vec<usize>`.
- Join all worker threads before returning.

Do not use `Arc<Mutex<_>>` for the result; use the channel to transfer values
back to the calling thread.

## Expected behavior

Three workers sending two values each produce two copies of each worker index:
`[0, 0, 1, 1, 2, 2]` after sorting. Channel receive order is not guaranteed.

## Run

```bash
make 02_concurrency_05_threads_channels
```

## Follow-up

After implementation, explain why the original sender must be dropped, why
`sender.clone()` is needed, and how channel closure ends the receiver loop.
