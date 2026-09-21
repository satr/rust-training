#[derive(Debug, PartialEq)]
struct Reading {
    device_id: String,
    value: i32,
}

#[derive(Debug, Default)]
struct ReadingService {
    readings: Vec<Reading>,
}

impl ReadingService {
    fn new() -> Self {
        ReadingService {
            readings: Vec::<Reading>::new(),
        }
    }

    fn ingest(&mut self, message: &str) -> Result<(), String> {
        let (device_id, raw_value) = message
            .trim()
            .split_once(',')
            .ok_or_else(|| "missing comma")?;
        let value = raw_value
            .parse::<i32>()
            .map_err(|_| String::from("invalid value"))?;
        self.readings.push(Reading {
            device_id: device_id.to_owned(),
            value,
        });
        Ok(())
    }

    fn count(&self) -> usize {
        self.readings.len()
    }

    fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum: i32 = self.readings.iter().map(|reading| reading.value).sum();
        Some(sum as f64 / self.readings.len() as f64)
    }
}

fn main() {
    let mut service = ReadingService::new();
    service.ingest("sensor-1,18").unwrap();
    service.ingest("sensor-2,22").unwrap();
    println!("count: {}", service.count());
    println!("average: {:?}", service.average());
}

#[cfg(test)]
mod tests {
    use super::ReadingService;

    #[test]
    fn ingests_readings_and_reports_statistics() {
        let mut service = ReadingService::new();
        assert_eq!(service.ingest("sensor-1,18"), Ok(()));
        assert_eq!(service.ingest("sensor-2,22"), Ok(()));
        assert_eq!(service.count(), 2);
        assert_eq!(service.average(), Some(20.0));
    }

    #[test]
    fn rejects_invalid_messages_without_changing_state() {
        let mut service = ReadingService::new();
        assert!(service.ingest("broken").is_err());
        assert_eq!(service.count(), 0);
        assert_eq!(service.average(), None);
    }

    #[test]
    fn supports_default_construction() {
        let service = ReadingService::default();
        assert_eq!(service.count(), 0);
    }
}
