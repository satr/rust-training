use std::fmt;

#[derive(Debug, PartialEq)]
enum ReadingError {
    MissingSeparator,
    EmptyDeviceId,
    InvalidValue,
}

impl fmt::Display for ReadingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadingError::MissingSeparator => {
                write!(formatter, "missing separator ','")
            }
            ReadingError::EmptyDeviceId => {
                write!(formatter, "device ID is empty")
            }
            ReadingError::InvalidValue => {
                write!(formatter, "value is not a valid integer")
            }
        }
    }
}

impl std::error::Error for ReadingError {}

fn main() {
    let errors = [
        ReadingError::MissingSeparator,
        ReadingError::EmptyDeviceId,
        ReadingError::InvalidValue,
    ];

    for error in errors {
        println!("{error}");
    }
}

#[cfg(test)]
mod tests {
    use super::ReadingError;

    #[test]
    fn formats_missing_separator() {
        assert_eq!(
            ReadingError::MissingSeparator.to_string(),
            "missing separator ','"
        );
    }

    #[test]
    fn formats_empty_device_id() {
        assert_eq!(
            ReadingError::EmptyDeviceId.to_string(),
            "device ID is empty"
        );
    }

    #[test]
    fn formats_invalid_value() {
        assert_eq!(
            ReadingError::InvalidValue.to_string(),
            "value is not a valid integer"
        );
    }
}
