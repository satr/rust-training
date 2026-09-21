# Exercise 19: MQTT to PostgreSQL service flow

## Theory

A backend service commonly sits between an external message and a repository:

1. Validate and parse the message boundary.
2. Convert valid input into a domain value.
3. Persist the domain value with parameterized SQL.
4. Map infrastructure failures into the service's error type.

For this task, the MQTT-style topic is:

```text
devices/{device_id}/readings
```

The payload is a decimal `i32`. Topic segments are borrowed while parsing, but
the returned `Reading` owns its `device_id` as a `String` so it can safely be
passed to the asynchronous database operation.

## Task

Implement both functions in `main.rs`.

`parse_mqtt_reading` must:

- Split the topic into segments.
- Require exactly `devices`, a non-empty device ID, and `readings`.
- Parse the payload as `i32`.
- Return `InvalidTopic` or `InvalidPayload` as appropriate.

`process_mqtt_message` must:

- Call `parse_mqtt_reading` and propagate validation errors.
- Insert the resulting reading with `INSERT_READING_COMMAND`.
- Bind values with `.bind(...)` rather than formatting SQL strings.
- Map the SQLx failure to `ServiceError::Database`.

## Expected behavior

- Valid input stores one row.
- Invalid topics never reach PostgreSQL.
- Invalid payloads return `InvalidPayload`.
- Database failures are exposed as `ServiceError::Database`.

## Run

```bash
make 04_mqtt_02_mqtt_postgres_service
make 04_mqtt_02_mqtt_postgres_service_infra
```

The infrastructure target runs the ignored PostgreSQL round-trip test in the
temporary Docker database.

## Follow-up

After implementation, explain why parsing should happen before opening or
using a database connection, and why the service returns an owned `String` for
the device ID.
