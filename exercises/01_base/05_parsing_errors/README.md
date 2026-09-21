# Exercise 05: Custom parsing errors

## Theory

Returning `String` errors is useful for quick experiments, but backend code usually benefits from a custom error enum. A custom enum lets callers distinguish failures programmatically:

```rust
enum ReadingError {
    MissingSeparator,
    EmptyDeviceId,
    InvalidValue,
}
```

The function can then return:

```rust
Result<Reading, ReadingError>
```

Callers can handle each case with `match` or tests can verify the exact error variant. `#[derive(Debug, PartialEq)]` makes values printable and comparable in tests.

## Task

Implement `parse_reading` in [`main.rs`](main.rs).

Parse input in the format `"sensor-1,42"` and return a `Reading` containing an owned `String` device ID and an `i32` value.

Return these specific errors:

- Missing comma → `ReadingError::MissingSeparator`
- Empty device ID → `ReadingError::EmptyDeviceId`
- Invalid integer → `ReadingError::InvalidValue`

The unit tests in the Rust file define the expected behavior.

## Run

```bash
cargo check --bin 05_parsing_errors
cargo test --bin 05_parsing_errors
```

Or use the Makefile:

```bash
make 01_base_05_parsing_errors
```

## Follow-up

The implementation matches the requirements. It converts the missing-separator `Option` into a `Result` with `ok_or_else`, returns a specific error for an empty device ID, and maps integer parsing failures to `ReadingError::InvalidValue`.

The returned `Reading` owns the device ID through `.to_owned()`. `#[derive(PartialEq)]` allows the tests to compare both successful readings and exact error variants. Both tests pass with `make 01_base_05_parsing_errors`.
