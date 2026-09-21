# Exercise 30: Tokio cancellation with `select!`

## Theory

Rust futures are cancelled cooperatively. Dropping a future stops polling it;
the future cannot be interrupted while it is executing blocking synchronous
code. Tokio's `select!` waits for several async branches and runs the first
branch that becomes ready:

```rust
tokio::select! {
    value = do_work() => println!("finished: {value}"),
    _ = cancellation => println!("cancelled"),
}
```

For this exercise, create the operation being cancelled as an async block:

```rust
let worker = async move {
    let mut collected = Vec::new();

    for value in values {
        collected.push(value);
        tokio::task::yield_now().await;
    }

    collected
};
```

This `worker` is a future, not necessarily a separately spawned Tokio task.
It is lazy and starts running only when `select!` polls it. `async move` moves
the input vector into the future so the future owns everything it needs. The
loop pushes values in their original order and the final expression becomes
the future's result.

Use the worker in `select!` by awaiting it as one branch and polling the
cancellation receiver as the other:

```rust
tokio::select! {
    result = worker => result,
    _ = &mut cancellation => Vec::new(),
}
```

`yield_now().await` voluntarily gives the Tokio runtime a chance to poll other
ready futures, including the cancellation branch. It is useful here because
the exercise demonstrates cooperative cancellation. Without an `.await`, a
worker that performs all its work in one synchronous stretch may finish before
the cancellation branch can run. `yield_now` is not a general replacement for
breaking up expensive CPU work; for genuinely blocking or CPU-heavy work, use
appropriate blocking APIs or a dedicated worker thread pool.

`tokio::sync::oneshot` is useful for one cancellation event. Its sender sends
exactly one value, while its receiver is awaited by the operation being
cancelled. The receiver must usually be mutable when passed to `select!`,
because selecting over a future may poll it more than once.

When a `select!` branch completes, the other branch is dropped. This is the
usual cancellation mechanism for an async operation. Make sure the work
future owns its resources and does not hold a lock or partially update shared
state in a way that cannot be safely abandoned.

This is similar to a C# `CancellationToken` raced against a `Task`, or a Go
`context.Context` case in a `select`. Rust makes the cancellation future and
the ownership of the operation explicit.

## Task

Implement `collect_until_cancelled` in `main.rs`:

- Create one async worker future that collects the supplied values in order.
- Race that worker against the `oneshot::Receiver<()>` with `tokio::select!`.
- Return the worker's collected values if it finishes first.
- Return an empty vector if cancellation arrives first.
- Ensure the cancellation receiver is handled in a way that allows it to be
  polled by `select!`.

Do not use threads, `Arc`, `Mutex`, channels other than the supplied oneshot
receiver, or blocking sleeps.

## Expected behavior

Without cancellation, `[10, 20, 30]` is returned unchanged. If the
cancellation sender has already sent its signal, the result is empty. Empty
work also returns an empty vector.

## Run

```bash
make 02_concurrency_09_tokio_cancellation
```

## Follow-up

Explain cooperative cancellation, why the losing `select!` branch is dropped,
why blocking code cannot be reliably cancelled, and how this pattern maps to
C# cancellation tokens and Go contexts.
