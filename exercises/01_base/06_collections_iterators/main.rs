#[derive(Debug, PartialEq, PartialOrd)]
struct Reading {
    device_id: String,
    value: i32,
}

fn accepted_device_ids(readings: &[Reading], minimum: i32) -> Vec<String> {
    // Return device IDs for readings whose value is at least minimum.
    // Preserve input order and return owned strings.
    readings
        .iter()
        .filter(|reading| reading.value >= minimum)
        .map(|reading| reading.device_id.clone())
        .collect::<Vec<String>>()
}

fn average_at_or_above(readings: &[Reading], minimum: i32) -> Option<f64> {
    // Return the average of values at least minimum, or None when no reading matches.
    let filtered_list: Vec<i32> = readings
        .iter()
        .filter(|reading| reading.value >= minimum)
        .map(|reading| reading.value)
        .collect();
    if filtered_list.is_empty() {
        return None;
    }
    let sum: i32 = filtered_list.iter().sum();
    Some(sum as f64 / filtered_list.len() as f64)
}

fn main() {
    let readings = vec![
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
    ];

    println!("accepted: {:?}", accepted_device_ids(&readings, 10));
    println!("average: {:?}", average_at_or_above(&readings, 10));
}

#[cfg(test)]
mod tests {
    use super::{accepted_device_ids, average_at_or_above, Reading};

    fn sample_readings() -> Vec<Reading> {
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
            Reading {
                device_id: String::from("sensor-4"),
                value: 10,
            },
        ]
    }

    #[test]
    fn filters_ids_in_input_order() {
        let readings = sample_readings();

        assert_eq!(
            accepted_device_ids(&readings, 10),
            vec![
                String::from("sensor-1"),
                String::from("sensor-3"),
                String::from("sensor-4")
            ]
        );
    }

    #[test]
    fn returns_empty_ids_when_nothing_matches() {
        let readings = sample_readings();

        assert_eq!(accepted_device_ids(&readings, 30), Vec::<String>::new());
    }

    #[test]
    fn averages_matching_values() {
        let readings = sample_readings();

        assert_eq!(average_at_or_above(&readings, 10), Some(53.0 / 3.0));
    }

    #[test]
    fn returns_none_when_nothing_matches() {
        let readings = sample_readings();

        assert_eq!(average_at_or_above(&readings, 30), None);
    }
}
