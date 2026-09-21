#[derive(Debug, PartialEq)]
enum Quality {
    Good,
    TooLow,
    TooHigh,
}

fn classify(value: i32) -> Quality {
    todo!()
}

fn describe(quality: &Quality) -> &str {
    todo!()
}

fn main() {
    for value in [-10, 0, 42, 100, 120] {
        let quality = classify(value);
        println!("{value}: {quality:?} ({})", describe(&quality));
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, describe, Quality};

    #[test]
    fn expected_behavior() {
        assert!(matches!(classify(-1), Quality::TooLow));
        assert!(matches!(classify(0), Quality::Good));
        assert!(matches!(classify(100), Quality::Good));
        assert!(matches!(classify(101), Quality::TooHigh));
        assert_eq!(describe(&Quality::Good), "accepted");
        assert_eq!(describe(&Quality::TooLow), "below range");
        assert_eq!(describe(&Quality::TooHigh), "above range");
    }
}
