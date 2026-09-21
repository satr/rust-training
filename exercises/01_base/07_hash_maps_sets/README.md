# Exercise 07: Hash maps and sets

## Theory

Rust's standard library provides hash-based collections for common coding algorithms:

- `HashSet<T>` stores unique values and supports fast average-case membership checks.
- `HashMap<K, V>` stores values by key and supports fast average-case lookup and update.

The usual duplicate-detection algorithm keeps values already seen:

```rust
let mut seen = HashSet::new();
for &value in values {
    if !seen.insert(value) {
        return Some(value);
    }
}
None
```

`HashSet::insert` returns `true` when the value was newly inserted and `false` when it was already present. This gives an efficient one-pass solution with average `O(n)` time and `O(n)` extra space.

For frequency counting, `HashMap::entry` handles the lookup and update together:

```rust
let mut counts = HashMap::new();
for &word in words {
    *counts.entry(word.to_owned()).or_insert(0) += 1;
}
```

Because `words` has type `&[&str]`, `for word in words` would make `word` an `&&str`. Writing `for &word in words` removes one reference and gives `word: &str`. Since the result must own its keys, convert each word with `.to_owned()` before inserting it into the map.

## Task

Implement the two functions in [`main.rs`](main.rs):

```rust
fn first_duplicate(values: &[i32]) -> Option<i32>
fn word_frequencies(words: &[&str]) -> HashMap<String, usize>
```

Requirements:

- `first_duplicate` returns the first value encountered for the second time.
- `first_duplicate` returns `None` when all values are unique.
- `word_frequencies` counts every word and returns owned `String` keys.
- An empty input returns an empty map.
- Do not mutate or consume the input slices.

The unit tests in the Rust file define the expected behavior.

## Complexity

Both algorithms should run in average `O(n)` time. The set or map uses `O(n)` additional space.

## Run

```bash
cargo check --bin 07_hash_maps_sets
cargo test --bin 07_hash_maps_sets
```

Or use the Makefile:

```bash
make 01_base_07_hash_maps_sets
```

## Follow-up

The implementation is correct and all four tests pass.

For duplicate detection:

```rust
for &value in values {
    if !seen.insert(value) {
        return Some(value);
    }
}
```

Iterating over `&[i32]` normally yields `&i32`; `for &value` destructures the reference and gives an `i32`. `HashSet::insert` returns `true` for a newly inserted value and `false` when the value was already present. Therefore, the first `false` identifies the first duplicate in encounter order.

For frequency counting:

```rust
for &word in words {
    *word_counts.entry(word.to_owned()).or_insert(0) += 1;
}
```

Here `for &word` converts `&&str` into `&str`, and `.to_owned()` creates the required `String` key. `entry(...).or_insert(0)` returns `&mut usize`; the leading `*` dereferences that mutable reference so the count itself can be incremented.

The algorithms use average `O(n)` time and `O(n)` additional space.
