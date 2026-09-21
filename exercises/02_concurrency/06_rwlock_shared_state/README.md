# Exercise 27: `RwLock` for shared read-heavy state

## Theory

`RwLock<T>` allows many readers or one writer:

```rust
let state = Arc::new(RwLock::new(vec![10, 20]));

let read_guard = state.read().unwrap();
let sum: i32 = read_guard.iter().sum();
```

The read guard keeps a shared borrow of the protected value until the guard is
dropped. A write guard provides exclusive mutable access:

```rust
let mut write_guard = state.write().unwrap();
write_guard.push(30);
```

`Arc` shares ownership between threads, while `RwLock` coordinates access to
the shared value. This differs from `Arc<Mutex<T>>`: a mutex permits only one
guard at a time, whereas an `RwLock` can allow concurrent read guards. Writers
still exclude all readers and other writers.

Lock acquisition returns `Result` because a lock can be poisoned if another
thread panics while holding it. For this exercise, `unwrap()` is acceptable in
the small test program; production code should decide how to handle poisoning.

## Task

Implement `snapshot_sum_after_updates` in `main.rs`:

- Wrap `initial` in `Arc<RwLock<Vec<i32>>>`.
- Spawn one writer thread that acquires a write lock and appends every value in
  `updates`.
- Join the writer before starting readers so every reader observes the final
  state.
- Spawn `reader_count` reader threads. Each reader must acquire a read lock and
  return the sum of the values.
- Join the readers and return their sums in the resulting vector.

Use `Arc::clone` for thread ownership and `move` closures. Do not hold a lock
guard across a thread join or longer than necessary.

## Expected behavior

- Initial `[10, 20]` plus updates `[5, 15]` produces one sum of `50` per reader.
- Zero readers returns an empty vector.
- An empty initial state can be updated normally.

## Run

```bash
make 02_concurrency_06_rwlock_shared_state
```

## Follow-up

Explain when `RwLock` is preferable to `Mutex`, why `Arc` is needed here, what
the read and write guards do, and why joining the writer before spawning the
readers makes the test deterministic.

In this exercise, creating a writer thread and immediately joining it
provides no practical benefit. A synchronous update would be simpler and more
idiomatic:

```rust
let mut state = values.write().unwrap();
for value in updates {
    state.push(*value);
}
```

The writer thread exists only to practice:

- moving owned data into thread::spawn;
- sharing state with Arc;
- acquiring an exclusive RwLock write guard;
- joining a thread before using its result.

But because readers start only after the writer joins, there is no actual read/write
concurrency. For production code, synchronous mutation is preferable unless the writer
represents an independent producer.

A better concurrency exercise would either:

- let readers and the writer run concurrently and test snapshots at defined
  synchronization points; or

- model a long-lived writer thread receiving updates through a channel while reader
  threads query shared state.
