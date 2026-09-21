# Exercise 04: Reading quality

## Theory

An `enum` represents a value that can be one of several variants. This is useful for backend states, commands, message types, and domain errors:

```rust
enum Quality {
    Good,
    TooLow,
    TooHigh,
}
```

Rust uses `match` to handle enum variants explicitly:

```rust
fn describe(quality: &Quality) -> &str {
    match quality {
        Quality::Good => "accepted",
        Quality::TooLow => "below range",
        Quality::TooHigh => "above range",
    }
}
```

`match` is an expression: every branch must produce the same type. An enum can also carry data, which is useful for richer processing results and errors.

## Task

Implement the stub methods in [`main.rs`](main.rs):

```rust
#[derive(Debug)]
enum Quality {
    Good,
    TooLow,
    TooHigh,
}

fn classify(value: i32) -> Quality
fn describe(quality: &Quality) -> &str
```

Classification rules:

- Values from `0` through `100` are `Good`.
- Values below `0` are `TooLow`.
- Values above `100` are `TooHigh`.

`describe` should return:

- `Good` → `"accepted"`
- `TooLow` → `"below range"`
- `TooHigh` → `"above range"`

## Expected behavior

Running the program should produce descriptions equivalent to:

```text
value=-10 ... below range
value=0 ... accepted
value=42 ... accepted
value=100 ... accepted
value=120 ... above range
```

## Run

```bash
cargo check --bin 04_quality
make 01_base_04_quality
```

Or use the Makefile:

```bash
make 01_base_04_quality
```

The Rust file also contains unit tests for the classification boundaries and descriptions:

```bash
cargo test --bin 04_quality
```

The tests fail while the `todo!()` stubs remain and pass after the methods are implemented.

## Follow-up

The implementation matches the task exactly. `classify` uses match guards for the two out-of-range cases and lets `_` represent the remaining `0..=100` range:

```rust
match value {
    value if value < 0 => Quality::TooLow,
    value if value > 100 => Quality::TooHigh,
    _ => Quality::Good,
}
```

`describe` receives `&Quality`, so it borrows the enum instead of moving it. Its `match` is exhaustive: every `Quality` variant has a description. The boundary tests cover `-1`, `0`, `100`, and `101`, and both unit tests pass with `cargo test --bin 04_quality`.
