#[derive(Debug)]
struct Reading {
    device_id: String,
    value: i32,
}

fn parse_reading(input: &str) -> Result<Reading, String> {
    // Parse input in this format:
    // "sensor-1,42" -> Reading { device_id: "sensor-1", value: 42 }
    //
    // Return an error when:
    // - the comma is missing
    // - the device ID is empty
    // - the value is not a valid i32
    let (device_id, raw_value) = input
        .trim()
        .split_once(',')
        .ok_or_else(|| String::from("missing comma"))?;
    if device_id.is_empty() {
        return Err(String::from("empty device_id"));
    }
    let value = raw_value
        .parse::<i32>()
        .map_err(|_| String::from("invalid parsing"))?;
    Ok(Reading {
        device_id: device_id.to_owned(),
        value,
    })
}

fn main() {
    for input in [
        "sensor-1,42",
        "sensor-2,-5",
        "broken",
        ",42",
        "sensor-3,nope",
    ] {
        println!("{:?}", parse_reading(input));
    }
}

#[cfg(test)]
mod tests {
    use super::parse_reading;

    #[test]
    fn parses_reading_into_owned_model() {
        let reading = parse_reading("sensor-1,42").unwrap();

        assert_eq!(reading.device_id, "sensor-1");
        assert_eq!(reading.value, 42);
    }

    #[test]
    fn accepts_negative_values() {
        let reading = parse_reading("sensor-2,-5").unwrap();

        assert_eq!(reading.value, -5);
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(parse_reading("broken").is_err());
        assert!(parse_reading(",42").is_err());
        assert!(parse_reading("sensor-3,nope").is_err());
    }
}
