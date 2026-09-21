# Exercise 28: Tokio tasks and `JoinHandle`

## Theory

Tokio tasks are lightweight asynchronous units of work. Spawn one with
`tokio::spawn`:

```rust
let handle = tokio::spawn(async move {
    42
});

let value = handle.await.unwrap();
```

`tokio::spawn` returns a `JoinHandle<T>`. Awaiting the handle gives a
`Result<T, tokio::task::JoinError>` because the task may panic or be aborted.
The task must own everything it uses, so `async move` is commonly required;
borrowed local data usually cannot outlive the parent future.

Spawning a task does not mean that the work has completed. The parent must
keep and await each handle. A `Vec<JoinHandle<T>>` is useful when starting a
dynamic number of similar tasks and collecting their results later.

On Tokio's current-thread runtime, tasks share one executor thread and make
progress when they yield at `.await` points. On a multi-thread runtime, Tokio
may run tasks on different worker threads. This differs from
`std::thread::spawn`: Tokio tasks are cheaper, but blocking synchronous work
inside a task can prevent other tasks from making progress.

In C#, this is similar to starting several `Task<T>` values and awaiting them;
in Go, it is similar to launching goroutines and collecting results explicitly.
Rust additionally checks that values moved into spawned tasks satisfy the
required ownership and `Send` bounds.

## Task

Implement `collect_async_worker_values` in `main.rs`:

- Spawn `worker_count` Tokio tasks with `tokio::spawn`.
- Each task receives its worker index and produces that index
  `values_per_worker` times.
- Store every returned `JoinHandle<Vec<usize>>` in a vector.
- Await every handle and combine all task results into one `Vec<usize>`.
- Propagate task failures using `unwrap()` for this small exercise.

Do not use threads, channels, `Arc`, or shared mutable state. Each task should
return owned data to the parent future.

## Expected behavior

Three tasks producing two values each return two copies of every worker index.
The result order is not guaranteed, so callers may sort it before comparing:

```text
[0, 0, 1, 1, 2, 2]
```

Zero tasks and tasks configured to produce zero values return an empty vector.

## Run

```bash
make 02_concurrency_07_tokio_tasks
```

## Follow-up

Explain the difference between spawning a task and awaiting its
`JoinHandle`, why the closure uses `async move`, what `JoinError` represents,
and why blocking work should not run directly inside an async Tokio task.
