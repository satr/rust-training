# Exercise 11: Async reading service

## Theory

An `async fn` returns a future. Calling it does not immediately execute the whole operation; `.await` waits for the future to complete:

```rust
async fn load_value() -> i32 {
    42
}

let value = load_value().await;
```

Rust needs an async runtime to drive futures. Tokio provides one here:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // `.await` is allowed here.
}
```

Async tests use `#[tokio::test]`. The service still uses ordinary ownership rules: `&mut self` permits an async method to change state, while `&self` permits read-only access.

## Task

Implement the three methods in [`main.rs`](main.rs):

- `new` should create an empty service.
- `ingest` should parse an integer string and append it to `values`; invalid input must return an error without changing state.
- `average` should return the average value or `None` for an empty service.

Both methods are asynchronous, so callers must use `.await`. Implement only the active `.rs` file; the matching `_todo.rs` file is the reset copy.

## Run

```bash
cargo check --bin 11_async_reading_service
cargo test --bin 11_async_reading_service
make 01_base_11_async_reading_service
```

Or use the Makefile:

```bash
make 01_base_11_async_reading_service
```

## Follow-up

The implementation is correct and both tests pass.

Tokio provides the runtime that drives the futures returned by the `async` methods. `#[tokio::main]` starts that runtime for `main`, while `#[tokio::test]` starts one for each asynchronous test. Calling `ingest(...)` or `average()` creates a future; `.await` waits for its result.

At the average calculation:

```rust
let sum: i32 = self.values.iter().sum();
```

`.iter()` is explicit and clear: it borrows the vector and yields `&i32` values. `sum()` supports summing those references because `i32` is `Copy`. An alternative such as `.into_iter()` may also work in this borrowed context through `IntoIterator`, but calling `into_iter()` on an owned `Vec` normally consumes it. Therefore `.iter()` is the safer and more communicative choice for a read-only method.

This service can later await MQTT subscription or PostgreSQL query futures without changing its basic ownership model.
