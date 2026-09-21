#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

fn parse_reading(input: &str) -> Result<Reading, String> {
    todo!()
}

fn main() {
    for input in ["sensor-1,42", "broken", ",42", "sensor-3,nope"] {
        println!("{:?}", parse_reading(input));
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_reading, Reading};

    #[test]
    fn expected_behavior() {
        assert_eq!(
            parse_reading("sensor-1,42"),
            Ok(Reading {
                device_id: String::from("sensor-1"),
                value: 42,
            })
        );
        assert!(parse_reading("broken").is_err());
        assert!(parse_reading(",42").is_err());
        assert!(parse_reading("sensor-3,nope").is_err());
    }
}
