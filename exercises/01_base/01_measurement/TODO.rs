fn parse_measurement(input: &str) -> Result<i32, String> {
    todo!()
}

fn average(values: &[i32]) -> Option<f64> {
    todo!()
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
    fn expected_behavior() {
        assert_eq!(parse_measurement(" 42 "), Ok(42));
        assert!(parse_measurement("bad").is_err());
        assert_eq!(average(&[10, 20, 30]), Some(20.0));
        assert_eq!(average(&[]), None);
    }
}
