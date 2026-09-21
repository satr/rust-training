# Exercise 06: Collections and iterators

## Theory

`Vec<T>` is Rust's growable collection, similar to `List<T>` in C# or a slice backed by an array in Go. A function can borrow a vector through a slice such as `&[Reading]`, which allows it to read any vector-like collection without taking ownership.

Iterator adapters express a sequence transformation:

```rust
let doubled: Vec<i32> = values
    .iter()
    .filter(|&&value| value > 0)
    .map(|&value| value * 2)
    .collect();
```

- `.iter()` yields borrowed items and leaves the collection available to the caller.
- `.filter(...)` keeps matching items.
- `.map(...)` transforms each item.
- `.collect()` materializes the iterator into a collection; the type annotation tells Rust which collection to build.
- `.sum()` consumes an iterator and adds its items.

Iterators are lazy: the filtering and mapping work happens when a consuming operation such as `collect`, `sum`, or `count` is called. This is similar to LINQ in C#, but Rust's iterator types are checked at compile time and do not require a general-purpose query runtime.

## Worked example

For a borrowed slice of integers, this returns the positive values doubled:

```rust
fn positive_doubles(values: &[i32]) -> Vec<i32> {
    values.iter().filter(|&&value| value > 0).map(|&value| value * 2).collect()
}
```

The `&` patterns copy each `i32` out of the borrowed references. For non-`Copy` values such as `String`, use methods such as `.clone()` when the result must own a value.

## Correct expressions

For `accepted_device_ids`, the iterator borrows each `Reading`. Accessing `value` only reads through that borrow, while `.clone()` creates the owned `String` required by the returned vector:

```rust
readings
    .iter()
    .filter(|reading| reading.value >= minimum)
    .map(|reading| reading.device_id.clone())
    .collect::<Vec<String>>()
```

The same expression can use a type annotation instead of turbofish syntax:

```rust
let ids: Vec<String> = readings
    .iter()
    .filter(|reading| reading.value >= minimum)
    .map(|reading| reading.device_id.clone())
    .collect();
```

One clear implementation of the average first collects matching integer values, then sums borrowed values from that vector:

```rust
let values: Vec<i32> = readings
    .iter()
    .filter(|reading| reading.value >= minimum)
    .map(|reading| reading.value)
    .collect();

if values.is_empty() {
    None
} else {
    let sum: i32 = values.iter().sum();
    Some(sum as f64 / values.len() as f64)
}
```

The average can also avoid the temporary vector by folding directly into a sum and count:

```rust
let (sum, count) = readings
    .iter()
    .filter(|reading| reading.value >= minimum)
    .map(|reading| reading.value)
    .fold((0_i32, 0_usize), |(sum, count), value| {
        (sum + value, count + 1)
    });

if count == 0 {
    None
} else {
    Some(sum as f64 / count as f64)
}
```

In both versions, the integer-to-`f64` casts happen before division. Otherwise Rust would either reject the integer/float mismatch or perform integer division if both operands remained integers.

## Task

Implement the two functions in [`main.rs`](main.rs):

```rust
fn accepted_device_ids(readings: &[Reading], minimum: i32) -> Vec<String>
fn average_at_or_above(readings: &[Reading], minimum: i32) -> Option<f64>
```

Requirements:

- Keep only readings whose `value` is greater than or equal to `minimum`.
- `accepted_device_ids` must return owned device IDs in their original order.
- `average_at_or_above` must return the arithmetic mean of matching values.
- Return `None` when no values match.
- Do not mutate or consume the input slice.

The unit tests in the Rust file define the expected behavior.

## Run

```bash
cargo check --bin 06_collections_iterators
cargo test --bin 06_collections_iterators
```

Or use the Makefile:

```bash
make 01_base_06_collections_iterators
```

## Follow-up

The first implementation attempt was not compiling yet:

- `filter` receives `&&Reading`; use a borrowed pattern or field access without moving the `Reading` value.
- `accepted_device_ids` must clone each `device_id`, because the input slice is borrowed and the returned `Vec<String>` must own its strings.
- `let filtered_list: = ...` is missing a type. More importantly, a collected `Vec<i32>` cannot later be used as an iterator with `.map(...)`; either keep the iterator chain lazy or iterate over the collected vector.
- The average must convert the integer sum and count to `f64` before division.

The implementation was corrected by borrowing each reading during filtering, cloning `device_id` for the owned result, collecting matching values as `Vec<i32>`, summing through `.iter()`, and casting the sum and count to `f64` before division.

`cargo test --bin 06_collections_iterators` passes all four tests. Running the binary prints `accepted: ["sensor-1", "sensor-3"]` and `average: Some(21.5)`. `rustfmt --check` still reports formatting changes for the long iterator expressions; formatting the file is the remaining cleanup.
