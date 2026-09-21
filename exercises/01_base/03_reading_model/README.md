# Exercise 03: Reading model

## Theory

A `struct` groups related fields into a domain object. A struct can own data, such as a `String`, even when the constructor function receives borrowed inputs.

`split_once(',')` returns `Option<(&str, &str)>`: either a pair of borrowed string slices or `None`. `ok_or_else` converts that `Option` into a `Result`, allowing `?` to handle a missing separator.

Tuple destructuring extracts both parts:

```rust
let (device_id, raw_value) = ...;
```

## Task

Implement:

```rust
#[derive(Debug)]
struct Reading {
    device_id: String,
    value: i32,
}

fn parse_reading(input: &str) -> Result<Reading, String>
```

Parse input in the form `"sensor-1,42"`. Return an error when the comma is missing, the device ID is empty, or the value is not a valid `i32`.

## Follow-up and explanation

The first attempt treated the result of `split_once` as an array and tried to call `len` and use indexing. The actual type is `Option<(&str, &str)>`, so it must first be converted and destructured:

```rust
let (device_id, raw_value) = input
    .trim()
    .split_once(',')
    .ok_or_else(|| String::from("missing comma"))?;
```

The device ID is converted from a borrowed `&str` into an owned `String` with `.to_owned()`. The parsed integer is then stored in the struct. All valid and invalid examples compile and produce the expected `Ok(Reading { ... })` or `Err(...)` results.

## Run

```bash
make 01_base_03_reading_model
```
