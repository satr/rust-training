use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum BatchError {
    InvalidTopic,
    InvalidPayload,
}

fn parse_reading(topic: &str, payload: &str) -> Result<Reading, BatchError> {
    let mut parts = topic.trim().split('/');
    let prefix = parts.next();
    let device_id = parts.next();
    let suffix = parts.next();
    let extra = parts.next();
    if prefix != Some("devices")
        || device_id.is_none_or(str::is_empty)
        || suffix != Some("readings")
        || extra.is_some()
    {
        return Err(BatchError::InvalidTopic);
    }
    let device_id = device_id.ok_or(BatchError::InvalidTopic)?;
    let value = payload
        .trim()
        .parse::<i32>()
        .map_err(|_| BatchError::InvalidPayload)?;
    if value < 0 {
        return Err(BatchError::InvalidPayload);
    }
    Ok(Reading {
        device_id: device_id.to_owned(),
        value,
    })
}

fn deduplicate_readings(messages: &[(&str, &str)]) -> Result<Vec<Reading>, BatchError> {
    let map: HashMap<String, Reading> = messages
        .iter()
        .map(|(topic, payload)| parse_reading(topic, payload))
        .map(|result| result.map(|reading| (reading.device_id.clone(), reading)))
        .collect::<Result<_, _>>()?;
    let mut readings: Vec<Reading> = map.into_values().collect();
    readings.sort_by(|left, right| left.device_id.cmp(&right.device_id));
    Ok(readings)
}

fn main() {
    println!("MQTT batch deduplication exercise");
}

#[cfg(test)]
mod tests {
    use super::{deduplicate_readings, parse_reading, BatchError, Reading};

    #[test]
    fn parses_a_reading() {
        assert_eq!(
            parse_reading("devices/sensor-1/readings", "42"),
            Ok(Reading {
                device_id: "sensor-1".to_owned(),
                value: 42,
            })
        );
    }

    #[test]
    fn keeps_the_last_reading_for_each_device() {
        let messages = [
            ("devices/sensor-1/readings", "10"),
            ("devices/sensor-2/readings", "20"),
            ("devices/sensor-1/readings", "15"),
        ];

        assert_eq!(
            deduplicate_readings(&messages),
            Ok(vec![
                Reading {
                    device_id: "sensor-1".to_owned(),
                    value: 15,
                },
                Reading {
                    device_id: "sensor-2".to_owned(),
                    value: 20,
                },
            ])
        );
    }

    #[test]
    fn returns_an_error_without_partial_results() {
        let messages = [
            ("devices/sensor-1/readings", "10"),
            ("devices/sensor-2/readings", "bad"),
        ];

        assert_eq!(
            deduplicate_readings(&messages),
            Err(BatchError::InvalidPayload)
        );
    }
}
