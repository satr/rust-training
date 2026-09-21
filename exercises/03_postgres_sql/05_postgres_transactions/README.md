# Exercise 17: PostgreSQL transactions

## Theory

A transaction groups several database operations into one atomic unit. The
usual SQLx flow is:

```rust
let mut transaction = pool.begin().await?;
// execute queries using &mut *transaction
transaction.commit().await?;
```

If an error is returned before `commit`, dropping the transaction rolls it
back. This prevents a partially inserted batch from becoming visible.

## Task

Implement `insert_readings_transactionally` in `main.rs`.

- Start a transaction with `pool.begin().await?`.
- Insert every reading using parameter binding and the transaction executor.
- Commit only after every insert succeeds.
- Return errors with `?`.

## Run

```bash
make 03_postgres_sql_05_postgres_transactions
make 03_postgres_sql_05_postgres_transactions_infra
```

The infrastructure target runs the ignored PostgreSQL test in a temporary
Docker database and removes the database resources afterward.

## Follow-up

The implementation passed review. Committing inside the loop would create a
separate successful transaction for each row, so a later failure could leave
the batch partially inserted. Keeping one transaction open and committing
only after the loop makes the batch atomic.

SQLx commonly receives `&mut *transaction`: `*` dereferences the transaction
wrapper to its underlying connection, and `&mut` passes a mutable borrow to
the executor. The transaction remains alive and can be committed afterward.
