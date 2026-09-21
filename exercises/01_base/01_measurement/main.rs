fn parse_measurement(input: &str) -> Result<i32, String> {
    if let Ok(val) = input.trim().parse::<i32>() {
        Ok(val)
    } else {
        Err(String::from("Not a number"))
    }
}

fn average(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sum = 0.0;
    for &val in values {
        sum = sum + val as f64;
    }
    return Some(sum / values.len() as f64);
}

fn main() {
    println!("{:?}", parse_measurement(" 42 "));
    println!("{:?}", parse_measurement("bad"));
    println!("{:?}", average(&[10, 20, 30]));
    println!("{:?}", average(&[]));
}

#[cfg(test)]
mod tests {
    use super::{average, parse_measurement};

    #[test]
    fn parses_trimmed_measurements() {
        assert_eq!(parse_measurement(" 42 "), Ok(42));
    }

    #[test]
    fn rejects_invalid_measurements() {
        assert!(parse_measurement("bad").is_err());
        assert!(parse_measurement("").is_err());
    }

    #[test]
    fn averages_values() {
        assert_eq!(average(&[10, 20, 30]), Some(20.0));
        assert_eq!(average(&[]), None);
    }
}
