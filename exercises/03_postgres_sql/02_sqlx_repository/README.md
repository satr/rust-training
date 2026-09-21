# Exercise 14: SQLx PostgreSQL repository

## Theory

`sqlx` is an async Rust database library. A PostgreSQL connection pool is represented by `PgPool` and can be shared by repository methods.

`#[derive(sqlx::FromRow)]` tells SQLx how to construct a Rust struct from selected database columns:

```rust
#[derive(sqlx::FromRow)]
struct Reading {
    device_id: String,
    value: i32,
}
```

Parameterized queries bind values separately from SQL text:

```rust
sqlx::query(insert_sql())
    .bind(&reading.device_id)
    .bind(reading.value)
    .execute(&self.pool)
    .await?;
```

The repository owns or shares the pool and exposes domain-oriented methods such as `insert` and `list`. The tests in this task do not require a running database; a later iteration will add migrations and integration tests against PostgreSQL.

## Task

Implement the three methods on `ReadingRepository` in [`main.rs`](main.rs):

- `new` should store the provided `PgPool`.
- `insert` should execute `insert_sql()`, bind `device_id` and `value`, await the query, and return `Ok(())` on success.
- `list` should execute `select_sql()`, fetch all rows, and return `Vec<Reading>`.

Use `?` to propagate `sqlx::Error`. The SQL functions and tests are already provided.

No PostgreSQL server is needed for the unit tests, but the repository methods should be written as if they will be called with a live pool.

## Run

```bash
cargo check --bin 14_sqlx_repository
cargo test --bin 14_sqlx_repository
make 03_postgres_sql_02_sqlx_repository
```

Or use the Makefile:

```bash
make 03_postgres_sql_02_sqlx_repository
```

## Follow-up

The implementation compiles and the two query-contract tests pass. No PostgreSQL server is needed for these tests.

`&self.pool` borrows the pool because SQLx implements `Executor` for a pool reference; the repository keeps ownership of the pool for future operations. `insert` uses `.bind(...)` for parameterized values, awaits execution, discards the query result, and returns `Ok(())`. `list` uses `query_as::<_, Reading>` so SQLx maps each returned row through `FromRow`, then fetches all rows asynchronously.

The remaining warnings only indicate that the repository methods are not called by `main`; they are not correctness errors.
