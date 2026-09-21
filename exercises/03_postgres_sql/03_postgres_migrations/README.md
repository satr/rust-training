# Exercise 15: PostgreSQL migrations

## Theory

A migration changes database schema in a repeatable, versioned way. The schema for this exercise is:

```sql
CREATE TABLE IF NOT EXISTS readings (
    device_id TEXT NOT NULL,
    value INTEGER NOT NULL
)
```

The application can execute this statement through SQLx:

```rust
sqlx::query(CREATE_READINGS_TABLE)
    .execute(pool)
    .await?;
```

Normal unit tests should not require a running database. Database-dependent tests can be marked `#[ignore]` and run explicitly when `DATABASE_URL` points to a test PostgreSQL instance.

## Task

Implement `ensure_schema` in [`main.rs`](main.rs). It should execute `CREATE_READINGS_TABLE` using the supplied `&PgPool`, await the result, propagate `sqlx::Error` with `?`, and return `Ok(())`.

The normal test verifies the migration statement without a database. The ignored integration test verifies that the statement can be applied to a real PostgreSQL server.

## Run

Without PostgreSQL:

```bash
cargo test --bin 15_postgres_migrations
```

With PostgreSQL configured:

```bash
DATABASE_URL=postgres://user:password@localhost/rust_training \
    cargo test --bin 15_postgres_migrations -- --ignored
```

Or use the Makefile for the normal test:

```bash
make 03_postgres_sql_03_postgres_migrations
```

To start a temporary PostgreSQL container, run the ignored integration test,
and shut the container down afterward:

```bash
make 03_postgres_sql_03_postgres_migrations_infra
```

## Follow-up

Complete this section after review, including why migrations should be repeatable and how ignored integration tests differ from unit tests.
