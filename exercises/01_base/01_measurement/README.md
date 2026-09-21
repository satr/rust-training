# Exercise 01: Measurements

## Theory

Rust uses borrowed views for data that a function only needs to read:

- `&str` is a borrowed string slice.
- `&[i32]` is a borrowed slice of integers.
- `Result<T, E>` represents success (`Ok(T)`) or failure (`Err(E)`).
- `Option<T>` represents a value that may be absent (`Some(T)` or `None`).

Borrowing avoids unnecessary ownership transfers and copies.

## Task

Implement:

```rust
fn parse_measurement(input: &str) -> Result<i32, String>
fn average(values: &[i32]) -> Option<f64>
```

`parse_measurement` should trim and parse an integer, returning an error for invalid or empty input. `average` should return `None` for an empty slice and the arithmetic average otherwise.

## Follow-up and explanation

The first implementation of `average` had three compiler issues:

1. A `match` expression returned an `Option` but was not returned from the function.
2. The accumulator was inferred as `i32`, while the calculation needed `f64`.
3. The accumulator needed to be mutable.

The final implementation uses `let mut sum = 0.0`, iterates with `for &value in values`, and returns `Some(sum / values.len() as f64)`.

The exercise demonstrated parsing, numeric casts, mutation, borrowed inputs, `Result`, and `Option`.

## Run

```bash
make 01_base_01_measurement
```
