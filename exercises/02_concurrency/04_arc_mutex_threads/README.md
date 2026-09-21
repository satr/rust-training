# Exercise 25: `Arc<Mutex<T>>` and threads

## Theory

`Arc<T>` is atomically reference-counted shared ownership. It allows multiple
threads to own the same value. `Rc<T>` cannot be used for this because its
reference counter is not thread-safe.

`Mutex<T>` protects mutable data. A thread calls `lock()` to obtain temporary
exclusive access. The lock guard releases the mutex automatically when it goes
out of scope.

The common pattern for shared mutable state between threads is:

```rust
Arc<Mutex<T>>
```

`thread::spawn` requires data moved into the closure to be owned and safe to
send to another thread. `Arc::clone` gives each worker another owner, and
`join()` waits for workers to finish.

To repeat an operation `n` times, use an exclusive range:

```rust
for _ in 0..n {
    // repeated operation
}
```

`0..n` produces values from `0` through `n - 1`. The `_` means that the loop
value is intentionally unused. Use `0..=n` when the upper bound should be
included.

`thread::spawn` starts a new thread and returns a `JoinHandle`:

```rust
let handle = thread::spawn(|| {
    println!("worker");
});

handle.join().unwrap();
```

Call `join()` to wait for the worker to finish. A `move` closure transfers
captured owned values into the new thread:

```rust
let counter = Arc::clone(&counter);
let handle = thread::spawn(move || {
    let mut value = counter.lock().unwrap();
    *value += 1;
});
```

When starting multiple workers, store their `JoinHandle`s in a `Vec` and join
all of them after spawning. Each worker receives its own `Arc::clone`, while
all clones refer to the same mutex and counter.

## Task

Implement `increment_in_threads`:

- Create a shared `Arc<Mutex<usize>>` counter initialized to `0`.
- Spawn `thread_count` worker threads.
- Each worker increments the counter `increments_per_thread` times.
- Join every worker before returning the final counter value.
- Unwrap or otherwise handle thread and mutex errors appropriately.

Do not use a non-thread-safe shared mutable variable. The final result must be
deterministic regardless of thread scheduling.

## Expected behavior

Four workers performing twenty-five increments each return `100`. Zero workers
or zero increments return `0`.

## Run

```bash
make 02_concurrency_04_arc_mutex_threads
```

## Follow-up

After implementation, explain why `Arc::clone` is needed, what `Mutex::lock()`
returns, when the lock is released, and what `thread::JoinHandle::join()` does.

### The `E0597` mutex-guard lifetime error

This can produce a lifetime error at the end of the function:

```rust
*counter.lock().unwrap()
```

`lock()` returns a temporary `MutexGuard`. The guard borrows `counter` and is
dropped when its scope ends. In a final expression, the compiler may consider
that temporary alive while local variables, including `counter`, are being
dropped. That would allow the guard to outlive the value it borrows, so Rust
reports `E0597`.

Copy the protected `usize` into a local variable first. This makes the guard
finish its scope before the function returns:

```rust
let result = *counter.lock().unwrap();
result
```

An explicit block makes the lifetime even more visible:

```rust
let result = {
    let guard = counter.lock().unwrap();
    *guard
};

result
```

The returned value is a plain `usize`; it no longer borrows the mutex.
