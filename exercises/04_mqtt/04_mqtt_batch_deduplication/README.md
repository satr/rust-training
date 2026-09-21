# Exercise 21: Deduplicating an MQTT batch

## Theory

`HashMap<K, V>` stores one value per key. Inserting another value with the same
key replaces the previous value. This is useful when a batch can contain
several readings for the same device and the service should keep the latest
reading.

`HashMap` does not guarantee iteration order. If the function needs predictable
output, collect the values and sort them before returning them.

## Task

Implement both functions in `main.rs`:

- `parse_reading` must accept exactly `devices/{non-empty-device-id}/readings`.
- Parse a non-negative `i32` payload and map invalid input to `BatchError`.
- `deduplicate_readings` must parse every message before returning.
- Keep only the last reading for each device using a `HashMap`.
- Return the final readings sorted by `device_id`.
- If any message is invalid, return the error and no partial result.

## Expected behavior

The sample batch produces `sensor-1 = 15` and `sensor-2 = 20`. The invalid
message test returns `Err(BatchError::InvalidPayload)`.

## Run

```bash
make 04_mqtt_04_mqtt_batch_deduplication
```

## Follow-up

After implementation, explain why inserting the same key replaces the previous
value, why `HashMap` output must be sorted for deterministic tests, and how
parsing into `Result<Vec<Reading>, BatchError>` prevents partial results.

## Outcome

Completed. The implementation validates each MQTT message, collects owned
readings, keeps the latest reading per device with `HashMap::insert`, sorts the
result by device ID, and returns no partial result when parsing fails. All
three unit tests pass.
