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
    todo!()
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
    fn expected_behavior() {
        assert_eq!(
            parse_reading("sensor-1,42"),
            Ok(Reading {
                device_id: String::from("sensor-1"),
                value: 42,
            })
        );
        assert_eq!(parse_reading("broken"), Err(ReadingError::MissingSeparator));
        assert_eq!(parse_reading(",42"), Err(ReadingError::EmptyDeviceId));
        assert_eq!(
            parse_reading("sensor-2,nope"),
            Err(ReadingError::InvalidValue)
        );
    }
}
