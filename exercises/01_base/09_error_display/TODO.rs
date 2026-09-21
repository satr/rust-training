use std::fmt;

#[derive(Debug, PartialEq)]
enum ReadingError {
    MissingSeparator,
    EmptyDeviceId,
    InvalidValue,
}

impl fmt::Display for ReadingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ReadingError {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::ReadingError;

    #[test]
    fn expected_behavior() {
        assert_eq!(
            ReadingError::MissingSeparator.to_string(),
            "missing separator ','"
        );
        assert_eq!(
            ReadingError::EmptyDeviceId.to_string(),
            "device ID is empty"
        );
        assert_eq!(
            ReadingError::InvalidValue.to_string(),
            "value is not a valid integer"
        );
    }
}
