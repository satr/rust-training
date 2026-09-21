fn add_reading(readings: &mut Vec<i32>, input: &str) -> Result<(), String> {
    let val = input
        .trim()
        .parse::<i32>()
        .map_err(|_| String::from("failed"))?;
    readings.push(val);
    Ok(())
}

fn total(readings: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in readings {
        sum += val;
    }
    sum
}

fn main() {
    let mut readings = Vec::new();

    println!("{:?}", add_reading(&mut readings, "10"));
    println!("{:?}", add_reading(&mut readings, "20"));
    println!("{:?}", add_reading(&mut readings, "bad"));

    println!("readings: {:?}", readings);
    println!("total: {}", total(&readings));
}

#[cfg(test)]
mod tests {
    use super::{add_reading, total};

    #[test]
    fn adds_valid_readings() {
        let mut readings = Vec::new();

        assert_eq!(add_reading(&mut readings, "10"), Ok(()));
        assert_eq!(add_reading(&mut readings, " 20 "), Ok(()));
        assert_eq!(readings, vec![10, 20]);
        assert_eq!(total(&readings), 30);
    }

    #[test]
    fn rejects_invalid_readings_without_mutating_the_vector() {
        let mut readings = vec![10];

        assert!(add_reading(&mut readings, "bad").is_err());
        assert_eq!(readings, vec![10]);
    }

    #[test]
    fn totals_empty_readings() {
        assert_eq!(total(&[]), 0);
    }
}
