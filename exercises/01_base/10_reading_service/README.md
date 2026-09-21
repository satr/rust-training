# Exercise 10: In-memory reading service

## Theory

A backend service usually owns state and exposes operations through methods. Rust methods are defined in an `impl` block:

```rust
impl ReadingService {
    fn count(&self) -> usize {
        self.readings.len()
    }
}
```

The receiver controls how the method uses the service:

- `&self` borrows the service immutably for read-only operations.
- `&mut self` borrows it mutably so the method can change internal state.
- `Self` refers to the type being implemented.

`#[derive(Default)]` provides a default constructor when all fields have defaults. `ReadingService::default()` can therefore create a service with an empty vector.

## Task

Implement the four methods in [`main.rs`](main.rs):

- `new` should create an empty service.
- `ingest` should parse `"sensor-1,42"`, own the device ID, and append a reading. Invalid input should return `Err` without changing state.
- `count` should return the number of stored readings.
- `average` should return the average value, or `None` when no readings exist.

The unit tests in both Rust files define the expected behavior. Implement only `10_reading_service.rs`; keep `10_reading_service_todo.rs` as the reset copy.

## Run

```bash
cargo check --bin 10_reading_service
cargo test --bin 10_reading_service
make 01_base_10_reading_service
```

Or use the Makefile:

```bash
make 01_base_10_reading_service
```

## Follow-up

The implementation is correct and all three tests pass.

`ingest` uses `&mut self` because it changes the service's internal `Vec<Reading>`. It parses the borrowed message, creates an owned `String` for `device_id`, pushes a new `Reading`, and returns `Ok(())`. If parsing fails, `?` returns early and the vector is not modified.

`count` and `average` use `&self` because they only inspect state. `average` maps each borrowed `Reading` to its `i32` value, sums those values, converts the sum to `f64`, and wraps the result in `Some`. Empty state returns `None`.

This service can later become an async MQTT or HTTP component while keeping the same ownership boundary: mutation happens through controlled methods, and read-only operations use shared borrows.
