# Exercise 22: `Box<T>` and recursive trees

## Theory

Rust must know the size of every value at compile time. A recursive enum such
as this would have infinite size:

```rust
enum Tree {
    Leaf(i32),
    Branch(Tree, Tree),
}
```

`Box<Tree>` fixes this because `Box` stores the recursive value on the heap,
while the enum stores only a pointer-sized value. The compiler can therefore
determine the size of `Tree`.

`Box<T>` owns exactly one value. It is useful for recursive data structures,
explicit heap allocation, and trait objects. It does not provide shared
ownership; that comes later with `Rc<T>` or `Arc<T>`.

Methods taking `&self` can recursively inspect the tree without taking
ownership of it. Pattern matching automatically gives access to the boxed
children through references.

## Task

Implement the three methods on `Tree` in `main.rs`:

- `sum` returns the sum of all leaf values.
- `leaf_count` returns the number of `Leaf` nodes.
- `depth` returns the maximum number of nodes from the root to a leaf.

Use recursive pattern matching. Do not consume the tree; all methods must use
`&self`.

## Expected behavior

For the sample tree, the sum is `35`, the leaf count is `3`, and the depth is
`3`. A single leaf has depth `1`.

## Run

```bash
make 02_concurrency_01_box_recursive_tree
```

## Follow-up

After implementation, explain why `Box` is required for recursive enums, what
`Box::new(...)` does, and why these methods borrow the tree instead of taking
ownership.
