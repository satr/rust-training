# Exercise 23: `Rc<T>` and shared ownership

## Theory

`Rc<T>` provides shared ownership for immutable values within one thread. Each
`Rc::clone` creates another owner of the same allocation and increments a
reference count; it does not deep-copy the contained value.

Use `Rc<T>` when several parts of a single-threaded program need to own the
same data. Use `Arc<T>` instead when the owners may live on different threads.
`Rc<T>` is not thread-safe and cannot be sent between threads.

An `Rc<Tree>` also allows a tree-like structure to share one subtree:

```rust
let shared = Rc::new(Tree::Leaf(7));
let branch = Tree::Branch {
    left: Rc::clone(&shared),
    right: Rc::clone(&shared),
};
```

Both children point to the same allocation. The subtree is dropped only after
the last `Rc` owner disappears.

## Task

Implement `sum` and `leaf_count` in `main.rs` using recursive pattern matching.
The methods must borrow the tree with `&self` and work when both branches point
to the same `Rc<Tree>` allocation.

The tests also check `Rc::strong_count` and demonstrate that `Rc::clone` clones
the owner, not the tree data.

## Expected behavior

When the same `Leaf(7)` is used as both children, the logical tree contains two
leaf references: `sum()` returns `14` and `leaf_count()` returns `2`.

## Run

```bash
make 02_concurrency_02_rc_shared_tree
```

## Follow-up

After implementation, explain the difference between `Box::new` and
`Rc::clone`, why `Rc` is single-threaded, and when `Arc` would be required.
