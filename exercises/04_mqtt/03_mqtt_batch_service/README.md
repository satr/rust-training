# Exercise 20: MQTT batch service

## Theory

When a service receives several messages together, validate the complete batch
before changing the database. A useful iterator pattern is:

```rust
let readings: Vec<Reading> = messages
    .iter()
    .map(|(topic, payload)| parse_mqtt_reading(topic, payload))
    .collect::<Result<_, _>>()?;
```

`collect` stops at the first error and returns that error. Only after this
collection succeeds should the service begin its PostgreSQL transaction. The
database writes then use one transaction and one final `commit`, so a database
failure cannot leave a partial batch.

## Task

Implement both functions in `main.rs`.

`parse_mqtt_reading` must:

- Require exactly `devices/{non-empty-device-id}/readings`.
- Parse the payload as a non-negative `i32`.
- Return `InvalidTopic` or `InvalidPayload` as appropriate.

`process_mqtt_batch` must:

- Parse all messages before beginning a transaction.
- Collect owned `Reading` values with an iterator and `collect::<Result<_, _>>()?`.
- Insert every reading with parameter binding and `&mut *transaction`.
- Map database failures to `ServiceError::Database`.
- Commit only after every insert succeeds.

## Expected behavior

- A valid batch inserts every reading.
- An invalid topic or payload prevents all database writes.
- A database failure rolls back the complete batch.
- An empty batch succeeds without inserting rows.

## Run

```bash
make 04_mqtt_03_mqtt_batch_service
make 04_mqtt_03_mqtt_batch_service_infra
```

The infrastructure target runs the ignored PostgreSQL batch test in the
temporary Docker database.

## Follow-up

After implementation, explain why parsing the batch before `pool.begin()` is
useful, what `collect::<Result<_, _>>()?` returns, and why one commit belongs
after the insert loop.
