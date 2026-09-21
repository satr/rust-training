fn add_reading(readings: &mut Vec<i32>, input: &str) -> Result<(), String> {
    todo!()
}

fn total(readings: &[i32]) -> i32 {
    todo!()
}

fn main() {
    let mut readings = Vec::new();
    println!("{:?}", add_reading(&mut readings, "10"));
    println!("{:?}", add_reading(&mut readings, "bad"));
    println!("total: {}", total(&readings));
}

#[cfg(test)]
mod tests {
    use super::{add_reading, total};

    #[test]
    fn expected_behavior() {
        let mut readings = Vec::new();
        assert_eq!(add_reading(&mut readings, "10"), Ok(()));
        assert!(add_reading(&mut readings, "bad").is_err());
        assert_eq!(readings, vec![10]);
        assert_eq!(total(&readings), 10);
    }
}
