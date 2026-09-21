# Exercise 08: Traits and generic functions

## Theory

A Rust trait defines shared behavior, similar to an interface in C# or a capability contract in Go. Types implement the trait explicitly:

```rust
trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Reading {
    fn summary(&self) -> String {
        format!("{}: reading={}", self.device_id, self.value)
    }
}
```

The `&self` receiver borrows the object. The implementation can read its fields without taking ownership.

A generic function can accept any type implementing a trait:

```rust
fn summarize_all<T: Summary>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.summary()).collect()
}
```

This is compile-time polymorphism. The function works for every `T` that implements `Summary`, without requiring a common base class.

## Task

Implement the three `todo!()` methods in [`main.rs`](main.rs):

- `Summary for Reading` should return `"<device_id>: reading=<value>"`.
- `Summary for DeviceStatus` should return `"<device_id>: online"` or `"<device_id>: offline"`.
- `summarize_all` should call `summary()` for every item and collect the returned strings into a `Vec<String>`.

The unit tests in the Rust file define the expected behavior.

## Run

```bash
cargo check --bin 08_traits
cargo test --bin 08_traits
make 01_base_08_traits
```

Or use the Makefile:

```bash
make 01_base_08_traits
```

## Follow-up

The implementation is correct and all three tests pass.

`Reading` and `DeviceStatus` implement the same `Summary` trait while formatting their data differently. This is similar to a C# interface implementation. In Go, the equivalent behavior would be based on implicit satisfaction of an interface; Rust requires the `impl Summary for Type` declaration.

The generic bound:

```rust
fn summarize_all<T: Summary>(items: &[T]) -> Vec<String>
```

means that `T` may be any concrete type implementing `Summary`. The function borrows a slice, uses `.iter()` to borrow each item, calls the trait method, and collects each returned `String` into a new vector. `summary()` returns owned strings, so the result remains valid independently of the input slice.
