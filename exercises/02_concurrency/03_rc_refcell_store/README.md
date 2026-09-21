# Exercise 24: `Rc<RefCell<T>>` and interior mutability

## Theory

`Rc<T>` allows multiple owners, but shared `Rc<T>` data is normally accessed
immutably. `RefCell<T>` enables mutation through a shared reference by checking
borrowing rules at runtime instead of compile time.

The common single-threaded pattern is:

```rust
Rc<RefCell<T>>
```

`Rc` answers “how many owners are there?” and `RefCell` answers “can this owner
borrow the value mutably right now?” Use `borrow()` for a shared borrow and
`borrow_mut()` for a mutable borrow. A conflicting borrow causes a runtime
panic, so keep borrow scopes short.

`RefCell<T>` is not thread-safe. For shared mutable state across threads, the
corresponding pattern is usually `Arc<Mutex<T>>` or `Arc<RwLock<T>>`.

## Task

Implement:

- `add_reading`, mutating the shared store with `borrow_mut()`.
- `total`, reading the store with `borrow()` and summing all values.

Multiple `Rc` handles must observe the same underlying store.

## Expected behavior

Adding `10` through one handle and `20` through another produces a total of
`30` through either handle. An empty store has total `0`.

## Run

```bash
make 02_concurrency_03_rc_refcell_store
```

## Follow-up

After implementation, explain why `Rc` alone cannot mutate shared data, what
`borrow_mut()` does, and why `RefCell` is unsuitable for multithreaded state.
