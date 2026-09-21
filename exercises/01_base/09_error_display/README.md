# Exercise 09: Standard error formatting

## Theory

Rust separates an error's debug representation from its user-facing message:

- `Debug` is intended for developers and diagnostics.
- `Display` is intended for readable output and is used by `{error}` and `.to_string()`.
- Implementing `std::error::Error` marks a type as a standard error and allows it to work with common error-handling APIs.

`Display` is a trait. Its implementation receives a formatter and writes a message with `write!` or `formatter.write_str`:

```rust
impl fmt::Display for ReadingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadingError::MissingSeparator => {
                write!(formatter, "missing separator ','")
            }
            // other variants...
        }
    }
}
```

This is similar to implementing `ToString` or a formatting method in C#, while fitting into Rust's standard formatting and error ecosystem.

## Task

Implement `fmt` in [`main.rs`](main.rs).

Return these exact messages:

- `MissingSeparator` → `"missing separator ','"`
- `EmptyDeviceId` → `"device ID is empty"`
- `InvalidValue` → `"value is not a valid integer"`

The `std::error::Error` implementation is already provided. The unit tests define the expected behavior.

## Run

```bash
cargo check --bin 09_error_display
cargo test --bin 09_error_display
make 01_base_09_error_display
```

Or use the Makefile:

```bash
make 01_base_09_error_display
```

## Follow-up

The implementation is correct and all three tests pass.

`fmt` matches every error variant and writes the human-readable message with `write!`. The `Display` implementation is used by both `.to_string()` and the `{error}` formatting syntax. `Debug` remains separate so developers can choose a diagnostic representation without changing the user-facing message.

Implementing `std::error::Error` makes `ReadingError` compatible with Rust's standard error-handling conventions and allows it to be used as a source error when larger backend operations add context.
