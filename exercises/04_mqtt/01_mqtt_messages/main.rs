#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum MqttError {
    InvalidTopic,
    EmptyDeviceId,
    InvalidPayload,
}

fn parse_mqtt_message(topic: &str, payload: &str) -> Result<Reading, MqttError> {
    let mut parts = topic.split('/');
    let prefix = parts.next();
    let device_id = parts.next();
    let suffix = parts.next();
    let extra = parts.next();

    if prefix != Some("sensors") || suffix != Some("reading") || extra.is_some() {
        return Err(MqttError::InvalidTopic);
    }
    let device_id = device_id.ok_or(MqttError::InvalidTopic)?;
    if device_id.is_empty() {
        return Err(MqttError::EmptyDeviceId);
    }
    let value = payload
        .trim()
        .parse::<i32>()
        .map_err(|_| MqttError::InvalidPayload)?;
    Ok(Reading {
        device_id: device_id.to_owned(),
        value,
    })
}

fn main() {
    for (topic, payload) in [
        ("sensors/sensor-1/reading", "42"),
        ("sensors/sensor-2/reading", "-5"),
        ("invalid/topic", "10"),
        ("sensors//reading", "10"),
        ("sensors/sensor-3/reading", "bad"),
    ] {
        println!(
            "{topic} / {payload} -> {:?}",
            parse_mqtt_message(topic, payload)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_mqtt_message, MqttError, Reading};

    #[test]
    fn parses_valid_mqtt_messages() {
        assert_eq!(
            parse_mqtt_message("sensors/sensor-1/reading", "42"),
            Ok(Reading {
                device_id: String::from("sensor-1"),
                value: 42,
            })
        );
    }

    #[test]
    fn rejects_invalid_topics() {
        assert_eq!(
            parse_mqtt_message("invalid/topic", "10"),
            Err(MqttError::InvalidTopic)
        );
        assert_eq!(
            parse_mqtt_message("sensors//reading", "10"),
            Err(MqttError::EmptyDeviceId)
        );
    }

    #[test]
    fn rejects_invalid_payloads() {
        assert_eq!(
            parse_mqtt_message("sensors/sensor-3/reading", "bad"),
            Err(MqttError::InvalidPayload)
        );
    }
}
