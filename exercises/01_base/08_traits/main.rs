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
        format!("{}: reading={}", self.device_id, self.value)
    }
}

impl Summary for DeviceStatus {
    fn summary(&self) -> String {
        let status = match self.online {
            true => "online",
            false => "offline",
        };
        format!("{}: {}", self.device_id, status)
    }
}

fn summarize_all<T: Summary>(items: &[T]) -> Vec<String> {
    items.iter().map(|v| v.summary()).collect()
}

fn main() {
    let readings = vec![Reading {
        device_id: String::from("sensor-1"),
        value: 42,
    }];
    let statuses = vec![DeviceStatus {
        device_id: String::from("sensor-1"),
        online: true,
    }];

    println!("{:?}", summarize_all(&readings));
    println!("{:?}", summarize_all(&statuses));
}

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
