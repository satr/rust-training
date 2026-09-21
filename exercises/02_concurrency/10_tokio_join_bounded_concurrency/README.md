# Exercise 31: Tokio `join!` with bounded concurrency

## Theory

`tokio::join!` runs multiple async operations concurrently on the same task and
waits until all branches finish:

```rust
let (left, right) = tokio::join!(left_future(), right_future());
```

This is similar to `Task.WhenAll` in C# or waiting for multiple goroutines in
Go, but with explicit futures and ownership.

The names `left_future()` and `right_future()` represent two independent async
operations. For example, each operation might return a `Result`:

```rust
async fn left_future() -> Result<Vec<usize>, ProcessingError> {
    // ...
}

async fn right_future() -> Result<Vec<usize>, ProcessingError> {
    // ...
}
```

`join!` waits for both futures and returns both results. It does not stop early
when one future returns an error:

```rust
let (left_result, right_result) =
    tokio::join!(left_future(), right_future());

let left = left_result?;
let right = right_result?;
```

The `?` operators propagate an error after both operations have completed. If
the results need different handling, match them explicitly:

```rust
match tokio::join!(left_future(), right_future()) {
    (Ok(left), Ok(right)) => println!("{left:?} {right:?}"),
    (Err(error), _) | (_, Err(error)) => {
        println!("processing failed: {error:?}");
    }
}
```

Use `tokio::try_join!` when the other operation should be stopped as soon as
one operation fails:

```rust
let (left, right) = tokio::try_join!(left_future(), right_future())?;
```

`try_join!` returns the first error and drops the other future. Dropping a
future cancels it when it is being run directly by `try_join!`. However,
dropping a spawned `JoinHandle` does not stop the underlying Tokio task; call
`handle.abort()` when an already-spawned task must be cancelled explicitly.

When each branch needs to process many items, launching unlimited tasks can
create memory pressure and overload downstream systems. A bounded concurrency
pattern uses `tokio::sync::Semaphore` to cap how many tasks can be "in flight"
at once:

```rust
let permits = Arc::new(Semaphore::new(2));
let permit = permits.clone().acquire_owned().await.unwrap();
// process one item
drop(permit);
```

The permit is dropped automatically at the end of scope, allowing the next
waiting task to proceed. This is a standard backend pattern for protecting DB
pools, APIs, and other shared resources.

### Spawning tasks with a semaphore

To limit spawned tasks, share one semaphore with all workers using `Arc`. Each
worker must acquire a permit before starting its work:

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

let semaphore = Arc::new(Semaphore::new(3));
let mut handles = Vec::new();

for id in 0..10 {
    let semaphore = Arc::clone(&semaphore);

    handles.push(tokio::spawn(async move {
        let _permit = semaphore
            .acquire_owned()
            .await
            .expect("semaphore closed");

        println!("task {id} started");
        sleep(Duration::from_millis(500)).await;
        println!("task {id} finished");
    }));
}

for handle in handles {
    handle.await.expect("worker task panicked");
}
```

Here, at most three workers can hold a permit and run their work at the same
time. Other workers wait at `acquire_owned().await` until a permit is released.
The permit is released automatically when `_permit` is dropped at the end of
the task. Keeping the permit alive across the async operation is important;
releasing it immediately after acquiring it would remove the concurrency limit.

`acquire_owned()` is especially useful with `tokio::spawn`: it returns an owned
permit that can move into the `'static` spawned task. The ordinary `acquire()`
method returns a borrowed permit, which may not live long enough for a spawned
task. The semaphore itself is wrapped in `Arc` because every worker needs to
share the same permit counter.

Avoid blocking calls (`std::thread::sleep`, heavy CPU loops) inside async task
bodies. Blocking prevents the runtime from polling other futures. Prefer async
operations (or `spawn_blocking` for true blocking work).

## Task

Implement both functions in `main.rs`:

1. `process_batch_bounded(values, max_in_flight)`
2. `process_two_batches(left, right, max_in_flight)`

Requirements:

- Reject `max_in_flight == 0` with `ProcessingError::InvalidConcurrencyLimit`.
- `process_batch_bounded` should:
  - Use a semaphore to limit in-flight item-processing tasks.
  - Spawn one Tokio task per input value.
  - Call `async_double` for each value and collect results.
  - Return `ProcessingError::WorkerPanicked` if a worker task panics.
- `process_two_batches` should run the two batch processors concurrently using
  `tokio::join!` and return both result vectors.

Do not use `std::thread`, `Mutex`, `RwLock`, or blocking sleeps for this task.

## Expected behavior

- `process_two_batches(vec![1,2,3], vec![4,5], 2)` returns left values
  `[2,4,6]` and right values `[8,10]` (ordering within each batch is not
  guaranteed).
- Empty inputs return empty vectors.
- A zero concurrency limit returns `InvalidConcurrencyLimit`.

## Run

```bash
make 02_concurrency_10_tokio_join_bounded_concurrency
```

## Follow-up

Explain why `join!` is suitable for two independent branches, how semaphore
permits enforce backpressure, and why blocking work inside async tasks harms
runtime fairness.
