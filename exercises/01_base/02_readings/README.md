# Exercise 02: Mutable readings

## Theory

Rust values have an owner. Assigning an owned value such as `String` can move it to a new owner. Functions can borrow instead:

- `&T` is a shared, read-only borrow.
- `&mut T` is an exclusive, mutable borrow.
- Many shared borrows may coexist, but a mutable borrow cannot coexist with other active borrows.

`Result<(), String>` means an operation either succeeds without returning a value or fails with a message. The `?` operator returns the error immediately and unwraps the success value.

## Task

Implement:

```rust
fn add_reading(readings: &mut Vec<i32>, input: &str) -> Result<(), String>
fn total(readings: &[i32]) -> i32
```

`add_reading` should parse an input string and append the integer to the vector. `total` should calculate the sum without taking ownership of the vector.

## Follow-up and explanation

The first attempt had these issues:

- `input` was mistyped as `imput`.
- Parsing returns `Result`, so failure is `Err`, not `None`.
- `Vec::push` returns `()`, while the function must return `Result<(), String>`.
- Successful completion is written as `Ok(())`, not `Ok()`.
- A final expression must not have a semicolon when it is the function's return value.

The final implementation parses with `parse::<i32>()`, converts the parser error using `map_err`, uses `?` to return failures, mutably borrows the vector, and returns `Ok(())`.

## Run

```bash
make 01_base_02_readings
```
