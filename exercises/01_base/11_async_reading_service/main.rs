#[derive(Default, Debug, PartialEq)]
struct AsyncReadingService {
    values: Vec<i32>,
}

impl AsyncReadingService {
    fn new() -> Self {
        AsyncReadingService::default()
    }

    async fn ingest(&mut self, message: &str) -> Result<(), String> {
        let value = message.trim().parse::<i32>().map_err(|_| "broken")?;
        self.values.push(value);
        Ok(())
    }

    async fn average(&self) -> Option<f64> {
        if self.values.is_empty() {
            return None;
        }
        let sum: i32 = self.values.iter().sum();
        Some(sum as f64 / self.values.len() as f64)
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut service = AsyncReadingService::new();
    service.ingest("18").await.unwrap();
    service.ingest("22").await.unwrap();
    println!("average: {:?}", service.average().await);
}

#[cfg(test)]
mod tests {
    use super::AsyncReadingService;

    #[tokio::test(flavor = "current_thread")]
    async fn ingests_values_and_calculates_average() {
        let mut service = AsyncReadingService::new();

        assert_eq!(service.ingest("18").await, Ok(()));
        assert_eq!(service.ingest("22").await, Ok(()));
        assert_eq!(service.average().await, Some(20.0));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn rejects_invalid_values_without_changing_state() {
        let mut service = AsyncReadingService::new();

        assert!(service.ingest("broken").await.is_err());
        assert_eq!(service.average().await, None);
    }
}
