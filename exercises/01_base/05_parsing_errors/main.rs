#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum ReadingError {
    MissingSeparator,
    EmptyDeviceId,
    InvalidValue,
}

fn parse_reading(input: &str) -> Result<Reading, ReadingError> {
    // Parse input in the format "sensor-1,42".
    // Return a specific ReadingError for each invalid input case.
    let (raw_device_id, raw_value) = input
        .trim()
        .split_once(',')
        .ok_or_else(|| ReadingError::MissingSeparator)?;
    if raw_device_id.is_empty() {
        return Err(ReadingError::EmptyDeviceId);
    }
    let value = raw_value
        .parse::<i32>()
        .map_err(|_| ReadingError::InvalidValue)?;
    Ok(Reading {
        device_id: raw_device_id.to_owned(),
        value: value,
    })
}

fn main() {
    for input in ["sensor-1,42", "broken", ",42", "sensor-2,nope"] {
        println!("{input:?} -> {:?}", parse_reading(input));
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_reading, Reading, ReadingError};

    #[test]
    fn parses_valid_reading() {
        assert_eq!(
            parse_reading("sensor-1,42"),
            Ok(Reading {
                device_id: String::from("sensor-1"),
                value: 42,
            })
        );
    }

    #[test]
    fn returns_specific_errors() {
        assert_eq!(parse_reading("broken"), Err(ReadingError::MissingSeparator));
        assert_eq!(parse_reading(",42"), Err(ReadingError::EmptyDeviceId));
        assert_eq!(
            parse_reading("sensor-2,nope"),
            Err(ReadingError::InvalidValue)
        );
    }
}
