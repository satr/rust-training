# Exercise 16: PostgreSQL repository integration

## Theory

An integration test checks that several real components work together. Here,
SQLx, PostgreSQL, the schema, and repository functions are tested against a
real database. `#[ignore]` keeps the normal unit-test run independent of
external services; the infrastructure Make target runs it explicitly.

SQLx repository methods follow the same pattern as other fallible async Rust
functions:

```rust
sqlx::query("INSERT ...")
    .bind(value)
    .execute(pool)
    .await?;
```

## Task

Implement `ensure_schema`, `insert_reading`, and `list_readings` in
`main.rs`.

- Create the `readings` table using `CREATE_READINGS_TABLE`.
- Insert both fields with SQL parameters (`$1`, `$2`), not string formatting.
- Query all readings ordered by `device_id` and map rows into `Reading`.
- Propagate SQLx errors with `?`.

## Run

```bash
make 03_postgres_sql_04_postgres_repository_integration
make 03_postgres_sql_04_postgres_repository_integration_infra
```

The second command starts PostgreSQL, runs both regular and ignored tests, and
deletes the containers and volumes afterward.

## Follow-up

After review, record how parameter binding protects SQL queries and why the
integration test truncates its table before inserting test data.
