# Exercise 12: MQTT-style message parsing

## Theory

An MQTT message has a topic and a payload. A common application convention is to encode the device identity in the topic:

```text
sensors/<device-id>/reading
```

The broker transports bytes, but application code should validate and convert them into domain types as early as possible. This exercise models the topic and payload as `&str` so the focus stays on parsing and error handling.

Domain errors are represented with an enum so callers can distinguish an invalid topic from an invalid payload:

```rust
enum MqttError {
    InvalidTopic,
    EmptyDeviceId,
    InvalidPayload,
}
```

## Task

Implement `parse_mqtt_message` in [`main.rs`](main.rs).

Requirements:

- Accept topics exactly in the form `sensors/<device-id>/reading`.
- Return `InvalidTopic` when the topic structure is wrong.
- Return `EmptyDeviceId` when the middle topic segment is empty.
- Parse the payload as `i32` and return `InvalidPayload` when parsing fails.
- Return an owned `Reading` with an owned `String` device ID.

The unit tests in both Rust files define the expected behavior. Implement only the active `.rs` file; keep `_todo.rs` as the identical reset copy.

## Run

```bash
cargo check --bin 12_mqtt_messages
cargo test --bin 12_mqtt_messages
make 04_mqtt_01_mqtt_messages
```

Or use the Makefile:

```bash
make 04_mqtt_01_mqtt_messages
```

## Follow-up

The implementation is correct and all three tests pass.

The topic is parsed lazily with `split('/')` and successive `next()` calls. The parser validates the prefix, device ID segment, suffix, and absence of extra segments before parsing the payload. The line:

```rust
let device_id = device_id.ok_or(MqttError::InvalidTopic)?;
```

uses shadowing: the first `device_id` is an `Option<&str>`, while the new binding is a validated `&str`. `.to_owned()` then creates the owned `String` required by `Reading`.

This boundary parser can later receive topic and payload data from an MQTT client library, validate it immediately, and pass a typed `Reading` to the service layer.
