# Exercise 35: Tokio timeouts and retries

## Goal

Practice wrapping async work with a timeout and retrying only failures that are
safe to retry.

Backend services often call databases, HTTP APIs, queues, or device gateways.
Those calls can be slow, temporarily unavailable, or permanently invalid. A good
service puts a deadline around each attempt and retries only transient failures.

## Theory

`tokio::time::timeout` runs a future with a deadline:

```rust
let result = tokio::time::timeout(Duration::from_millis(50), some_future()).await;
```

The returned value has two layers:

```rust
Result<OutputOfFuture, tokio::time::error::Elapsed>
```

If the timeout expires, Tokio drops the future and returns `Err(Elapsed)`. For
ordinary async work, dropping the future cancels it cooperatively. This is
similar to a C# `CancellationToken` timeout or a Go `context.WithTimeout`, but
in Rust the cancellation point is expressed by future ownership: once the future
is dropped, it can no longer make progress.

When the inner future also returns a `Result`, you handle two error layers:

```rust
let value = tokio::time::timeout(deadline, call_backend())
    .await
    .map_err(|_| RequestError::TimedOut)??;
```

The first `?` handles timeout expiration after mapping it to your domain error.
The second `?` handles the backend call's own error.

Retries should be selective:

- Retry temporary failures and timeouts.
- Do not retry permanent failures.
- Stop after a bounded number of attempts.

Unbounded retries can overload a failing dependency and create retry storms.

## Provided code

`call_backend` simulates one backend attempt:

- `Success` waits for `delay`, then returns a value.
- `TemporaryFailure` waits for `delay`, then returns `TemporaryFailure`.
- `PermanentFailure` waits for `delay`, then returns `PermanentFailure`.

Do not edit `call_backend` or the tests.

## Task

Implement two functions in `main.rs`:

1. `request_with_timeout(attempt, per_attempt_timeout)`
2. `request_with_retries(attempts, per_attempt_timeout, max_attempts)`

Requirements:

- `request_with_timeout` must wrap `call_backend(attempt)` in
  `tokio::time::timeout`.
- Map elapsed timeout errors to `RequestError::TimedOut`.
- Preserve `TemporaryFailure` and `PermanentFailure` from `call_backend`.
- `request_with_retries` must try attempts in vector order.
- Retry only `TemporaryFailure` and `TimedOut`.
- Return immediately on `PermanentFailure`.
- Stop after `max_attempts` attempts.
- Return `RetryLimitExceeded` when retryable failures exhaust the allowed
  attempts or there are no attempts left.

## Expected behavior

- A fast successful request returns its value.
- A slow request returns `TimedOut`.
- A temporary failure followed by a success returns the successful value.
- A permanent failure stops retrying immediately.
- Retryable failures that exhaust the attempt limit return `RetryLimitExceeded`.

## Run

```bash
make 02_concurrency_14_tokio_timeouts_retries
```

## Follow-up

Explain why timeout errors and backend errors are separate layers, why retrying
permanent failures is dangerous, and how per-attempt timeouts help protect an
async backend service from slow dependencies.
