#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

fn ids_with_take_skip(readings: &[Reading], from_id: usize, to_id: usize) -> Vec<String> {
    readings
        .iter()
        .take(to_id)
        .skip(from_id)
        .map(|v| v.device_id.clone())
        .collect()
}

fn ids_with_slice(readings: &[Reading], from_id: usize, to_id: usize) -> Vec<String> {
    readings[from_id..to_id]
        .iter()
        .map(|v| v.device_id.clone())
        .collect()
}

fn main() {
    let readings = vec![
        Reading {
            device_id: String::from("sensor-1"),
            value: 18,
        },
        Reading {
            device_id: String::from("sensor-2"),
            value: 22,
        },
        Reading {
            device_id: String::from("sensor-3"),
            value: 25,
        },
    ];

    println!("{:?}", ids_with_take_skip(&readings, 1, 3));
    println!("{:?}", ids_with_slice(&readings, 1, 3));
}

#[cfg(test)]
mod tests {
    use super::{ids_with_slice, ids_with_take_skip, Reading};

    fn sample_readings() -> Vec<Reading> {
        vec![
            Reading {
                device_id: String::from("sensor-1"),
                value: 18,
            },
            Reading {
                device_id: String::from("sensor-2"),
                value: 22,
            },
            Reading {
                device_id: String::from("sensor-3"),
                value: 25,
            },
            Reading {
                device_id: String::from("sensor-4"),
                value: 31,
            },
        ]
    }

    #[test]
    fn take_then_skip_matches_slice_range() {
        let readings = sample_readings();
        let expected = vec![String::from("sensor-2"), String::from("sensor-3")];

        assert_eq!(ids_with_take_skip(&readings, 1, 3), expected);
        assert_eq!(ids_with_slice(&readings, 1, 3), expected);
    }

    #[test]
    fn range_end_is_exclusive() {
        let readings = sample_readings();

        assert_eq!(
            ids_with_take_skip(&readings, 0, 2),
            vec![String::from("sensor-1"), String::from("sensor-2")]
        );
        assert_eq!(
            ids_with_slice(&readings, 0, 2),
            vec![String::from("sensor-1"), String::from("sensor-2")]
        );
    }

    #[test]
    fn empty_ranges_are_supported() {
        let readings = sample_readings();

        assert_eq!(ids_with_take_skip(&readings, 2, 2), Vec::<String>::new());
        assert_eq!(ids_with_slice(&readings, 2, 2), Vec::<String>::new());
    }
}
