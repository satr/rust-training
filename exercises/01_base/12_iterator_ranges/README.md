# Exercise 12: Iterator ranges and slices

## Theory

Rust uses half-open ranges: `from_id..to_id` includes `from_id` and excludes
`to_id`. For example, `values[1..3]` selects indexes `1` and `2`.

For a slice, this range is equivalent to taking the first `to_id` items and
then skipping the first `from_id`:

```rust
values.iter().take(to_id).skip(from_id)
```

The order matters. `take(to_id).skip(from_id)` selects `[from_id..to_id]`,
while `skip(from_id).take(to_id)` selects a different range. This is useful
when an iterator is the input, but direct slicing is concise when random access
and a slice are already available.

Both approaches borrow the input. The task returns owned `String` values, so
each selected `device_id` must be cloned. The range arguments must satisfy
`0 <= from_id <= to_id <= readings.len()`.

## Worked example

Given `["a", "b", "c", "d"]`, both `items[1..3]` and
`items.iter().take(3).skip(1)` select `["b", "c"]`. The item at index `3` is
not included.

## Task

Implement both functions in [`main.rs`](main.rs):

```rust
fn ids_with_take_skip(
    readings: &[Reading],
    from_id: usize,
    to_id: usize,
) -> Vec<String>

fn ids_with_slice(
    readings: &[Reading],
    from_id: usize,
    to_id: usize,
) -> Vec<String>
```

Use `take(to_id).skip(from_id)` in the first function and
`readings[from_id..to_id]` in the second. Preserve order, return owned IDs, and
do not mutate or consume the input slice.

The tests cover equivalent results, an exclusive end, and an empty range.

## Run

```bash
cargo test --bin 01_base_12_iterator_ranges
make 01_base_12_iterator_ranges
```

## Follow-up

After implementing the two versions, compare their readability and consider
which one you would choose for a paginated API over an existing slice versus a
general iterator pipeline.
