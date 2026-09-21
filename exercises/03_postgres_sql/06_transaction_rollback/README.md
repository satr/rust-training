# Exercise 18: Transaction rollback

## Theory

A transaction is useful only when both success and failure are handled as one
unit. Start it with `pool.begin().await?`, execute every query through the
transaction, and call `commit` only after the whole operation succeeds.

If the function returns an error before `commit`, the transaction is dropped
and PostgreSQL rolls it back. Earlier successful statements in that transaction
are therefore not visible.

```rust
let mut transaction = pool.begin().await?;

for reading in readings {
    sqlx::query(INSERT_READING_COMMAND)
        .bind(&reading.device_id)
        .bind(reading.value)
        .execute(&mut *transaction)
        .await?;
}

transaction.commit().await?;
Ok(())
```

The `CHECK (value >= 0)` constraint gives the task a deterministic database
failure. A batch containing `-1` must leave zero rows behind.

## Task

Implement `insert_readings_transactionally` in `main.rs`:

- Begin one transaction.
- Insert every reading with parameter binding and `&mut *transaction`.
- Commit only after the loop completes.
- Propagate errors with `?`.

Do not call `commit` inside the loop. Do not insert through `pool` after the
transaction has started.

## Expected behavior

- A valid batch inserts every reading.
- A batch containing a negative value returns an error.
- When an insert fails, the count in the table remains zero.

## Run

```bash
make 03_postgres_sql_06_transaction_rollback
make 03_postgres_sql_06_transaction_rollback_infra
```

The infrastructure target starts the temporary PostgreSQL database, runs the
ignored integration tests, and removes its resources afterward.

The integration tests share one database table, so they use a process-local
mutex to prevent concurrent schema creation and truncation from racing.

## Follow-up

After implementing the task, record why the first valid insert is not visible
after the second insert violates the `CHECK` constraint. Also explain why
`transaction.commit().await?` must be after the loop.

## Outcome

The implementation starts one transaction, executes each insert through
`&mut *transaction`, propagates the first database error with `?`, and commits
only after the loop. The focused Cargo test and formatting checks pass. The
database integration tests remain ignored unless Docker is available.
