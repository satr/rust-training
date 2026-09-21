#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

fn accepted_device_ids(readings: &[Reading], minimum: i32) -> Vec<String> {
    todo!()
}

fn average_at_or_above(readings: &[Reading], minimum: i32) -> Option<f64> {
    todo!()
}

fn main() {
    let readings = vec![Reading {
        device_id: String::from("sensor-1"),
        value: 18,
    }];
    println!("{:?}", accepted_device_ids(&readings, 10));
    println!("{:?}", average_at_or_above(&readings, 10));
}

#[cfg(test)]
mod tests {
    use super::{accepted_device_ids, average_at_or_above, Reading};

    fn sample() -> Vec<Reading> {
        vec![
            Reading {
                device_id: String::from("sensor-1"),
                value: 18,
            },
            Reading {
                device_id: String::from("sensor-2"),
                value: 7,
            },
            Reading {
                device_id: String::from("sensor-3"),
                value: 25,
            },
        ]
    }

    #[test]
    fn expected_behavior() {
        let readings = sample();
        assert_eq!(
            accepted_device_ids(&readings, 10),
            vec![String::from("sensor-1"), String::from("sensor-3")]
        );
        assert_eq!(average_at_or_above(&readings, 10), Some(21.5));
        assert_eq!(average_at_or_above(&readings, 30), None);
    }
}
