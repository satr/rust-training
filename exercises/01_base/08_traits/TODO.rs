#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

#[derive(Debug, PartialEq)]
struct DeviceStatus {
    device_id: String,
    online: bool,
}

trait Summary {
    fn summary(&self) -> String;
}

impl Summary for Reading {
    fn summary(&self) -> String {
        todo!()
    }
}

impl Summary for DeviceStatus {
    fn summary(&self) -> String {
        todo!()
    }
}

fn summarize_all<T: Summary>(items: &[T]) -> Vec<String> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{summarize_all, DeviceStatus, Reading, Summary};

    #[test]
    fn expected_behavior() {
        let reading = Reading {
            device_id: String::from("sensor-1"),
            value: 42,
        };
        let status = DeviceStatus {
            device_id: String::from("sensor-1"),
            online: true,
        };
        assert_eq!(reading.summary(), "sensor-1: reading=42");
        assert_eq!(status.summary(), "sensor-1: online");
        assert_eq!(
            summarize_all(&[reading]),
            vec![String::from("sensor-1: reading=42")]
        );
    }
}
