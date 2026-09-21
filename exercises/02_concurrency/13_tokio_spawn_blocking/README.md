# Exercise 34: Tokio `spawn_blocking`

## Goal

Practice moving blocking or legacy synchronous work out of async tasks.

In this exercise, `parse_reading_blocking` is a provided synchronous function.
Pretend it is old code from another crate: it blocks the current thread briefly,
parses a reading, and may panic if that legacy code crashes. Your job is not to
rewrite that parser. Your job is to call it safely from async code.

## Theory

Tokio async tasks should avoid blocking the runtime thread. If an async function
directly runs CPU-heavy work, synchronous file IO, `std::thread::sleep`, or a
legacy parser that blocks, other futures on the same runtime thread cannot make
progress while that work is running.

Use `tokio::task::spawn_blocking` for this boundary:

```rust
let handle = tokio::task::spawn_blocking(move || {
    legacy_blocking_function()
});

let result = handle.await;
```

Important details:

- The closure is `move` because it runs on another thread and must own its data.
- The closure can return a `Result<T, E>`.
- `handle.await` returns `Result<T, JoinError>`.
- If the blocking closure panics, `handle.await` returns `Err(JoinError)`.
- After awaiting the handle, you often need two layers of error handling:
  `JoinError` for task failure, then your domain error for normal failures.

That shape usually looks like this:

```rust
let parsed = handle
    .await
    .map_err(|_| ProcessingError::WorkerPanicked)??;
```

The first `?` handles the `JoinError` after mapping it. The second `?` handles
the `Result` returned by your blocking closure.

This is similar to using `Task.Run` in C# for blocking work. In Go, you might
move blocking work to a separate goroutine, but Rust makes the ownership
boundary explicit: data moved into the blocking closure cannot also be used by
the async caller.

## Worked example

Here is a smaller version of the pattern:

```rust
async fn parse_one(raw: String) -> Result<usize, ProcessingError> {
    let handle = tokio::task::spawn_blocking(move || {
        raw.parse::<usize>()
            .map_err(|_| ProcessingError::ParseFailed)
    });

    handle
        .await
        .map_err(|_| ProcessingError::WorkerPanicked)?
}
```

Notice that `raw` is moved into the closure. The async function awaits the
`JoinHandle`, maps a panic into `WorkerPanicked`, and then returns the parser's
own `Result`.

## Task

Implement two functions in `main.rs`:

1. `parse_batch_off_runtime(values)`
2. `parse_two_batches(left, right)`

Requirements:

- `parse_batch_off_runtime` must call `tokio::task::spawn_blocking`.
- Move the whole `Vec<String>` into the blocking closure.
- Inside the blocking closure, call the provided `parse_reading_blocking` for
  each value.
- Preserve input order in the returned `Vec<usize>`.
- Return `ProcessingError::ParseFailed` if any reading is not a valid `usize`.
- Return `ProcessingError::WorkerPanicked` if the blocking task panics.
- `parse_two_batches` must run the left and right batch parsers concurrently
  with `tokio::join!`.

Do not edit `parse_reading_blocking` or the tests.

## Expected behavior

- `["10", "20", "30"]` returns `[10, 20, 30]`.
- `["10", "bad", "30"]` returns `ParseFailed`.
- `["10", "panic", "30"]` returns `WorkerPanicked`.
- Two valid batches return both parsed vectors.
- Empty batches return empty vectors.

## Run

```bash
make 02_concurrency_13_tokio_spawn_blocking
```

## Follow-up

Explain why the blocking parser should not run directly inside an async task,
why the closure passed to `spawn_blocking` uses `move`, and why awaiting the
`JoinHandle` creates a separate error layer from parsing failures.
