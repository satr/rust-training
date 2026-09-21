#[derive(Debug)]
enum Quality {
    Good,
    TooLow,
    TooHigh,
}

fn classify(value: i32) -> Quality {
    // Values 0..=100 are Good.
    // Values below 0 are TooLow.
    // Values above 100 are TooHigh.
    match value {
        value if value < 0 => Quality::TooLow,
        value if value > 100 => Quality::TooHigh,
        _ => Quality::Good,
    }
}

fn describe(quality: &Quality) -> &str {
    // Good    -> "accepted"
    // TooLow  -> "below range"
    // TooHigh -> "above range"
    match quality {
        Quality::Good => "accepted",
        Quality::TooLow => "below range",
        Quality::TooHigh => "above range",
    }
}

fn main() {
    for value in [-10, 0, 42, 100, 120] {
        let quality = classify(value);

        println!(
            "value={value}, quality={quality:?}, description={}",
            describe(&quality)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, describe, Quality};

    #[test]
    fn classifies_values_by_range() {
        assert!(matches!(classify(-1), Quality::TooLow));
        assert!(matches!(classify(0), Quality::Good));
        assert!(matches!(classify(100), Quality::Good));
        assert!(matches!(classify(101), Quality::TooHigh));
    }

    #[test]
    fn describes_quality() {
        assert_eq!(describe(&Quality::Good), "accepted");
        assert_eq!(describe(&Quality::TooLow), "below range");
        assert_eq!(describe(&Quality::TooHigh), "above range");
    }
}
