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
        todo!()
    }

    fn ingest(&mut self, message: &str) -> Result<(), String> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }

    fn average(&self) -> Option<f64> {
        todo!()
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
