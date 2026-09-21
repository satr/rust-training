# Exercise 13: PostgreSQL query boundaries

## Theory

PostgreSQL uses numbered parameters such as `$1` and `$2`:

```sql
INSERT INTO readings (device_id, value) VALUES ($1, $2)
```

The values are supplied separately by the database client library. This is parameterized SQL: input values are not concatenated into the SQL string, which avoids SQL injection and lets the driver handle encoding and escaping.

A repository usually keeps SQL statements near the code that executes them. Separating query definitions from domain data makes the database boundary easier to test before connecting to a real database.

## Task

Implement the two functions in [`main.rs`](main.rs):

- `insert_reading_sql` should return `INSERT INTO readings (device_id, value) VALUES ($1, $2)`.
- `select_readings_sql` should return `SELECT device_id, value FROM readings ORDER BY device_id`.

The tests verify the exact query strings. Do not interpolate values into SQL strings; later, `sqlx` will bind the `Reading` fields to `$1` and `$2` separately.

## Run

```bash
cargo check --bin 13_postgres_queries
cargo test --bin 13_postgres_queries
make 03_postgres_sql_01_postgres_queries
```

Or use the Makefile:

```bash
make 03_postgres_sql_01_postgres_queries
```

## Follow-up

The implementation is correct and both tests pass.

The SQL functions return string literals directly. A literal has type `&'static str`, so it matches the return type without allocating. Returning `&String` created inside the function would be invalid because the local `String` is dropped when the function returns. If dynamic SQL were required, the function would return an owned `String` instead.

The `$1` and `$2` placeholders keep values separate from SQL text. In the next database iteration, `sqlx` can bind `Reading` fields to those placeholders through a `PgPool` without string interpolation.
