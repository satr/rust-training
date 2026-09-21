use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Eq)]
enum ProcessingError {
    ParseFailed,
    WorkerPanicked,
}

#[allow(dead_code)]
fn parse_reading_blocking(value: String) -> Result<usize, ProcessingError> {
    thread::sleep(Duration::from_millis(1));

    if value == "panic" {
        panic!("legacy parser crashed");
    }

    value
        .parse::<usize>()
        .map_err(|_| ProcessingError::ParseFailed)
}

async fn parse_batch_off_runtime(values: Vec<String>) -> Result<Vec<usize>, ProcessingError> {
    let _ = values;
    todo!("move the batch into spawn_blocking, parse each value, then await the JoinHandle")
}

async fn parse_two_batches(
    left: Vec<String>,
    right: Vec<String>,
) -> Result<(Vec<usize>, Vec<usize>), ProcessingError> {
    let _ = (left, right);
    todo!("run both batch parsers concurrently with tokio::join!")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio spawn_blocking exercise");
}

#[cfg(test)]
mod tests {
    use super::{parse_batch_off_runtime, parse_two_batches, ProcessingError};

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[tokio::test(flavor = "current_thread")]
    async fn parses_one_batch_on_blocking_pool() {
        let readings = parse_batch_off_runtime(strings(&["10", "20", "30"]))
            .await
            .expect("valid readings should parse");

        assert_eq!(readings, vec![10, 20, 30]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn parses_two_batches_concurrently() {
        let (left, right) = parse_two_batches(strings(&["1", "2"]), strings(&["3", "4"]))
            .await
            .expect("valid batches should parse");

        assert_eq!(left, vec![1, 2]);
        assert_eq!(right, vec![3, 4]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_batches_return_empty_vectors() {
        let (left, right) = parse_two_batches(Vec::new(), Vec::new())
            .await
            .expect("empty batches should still succeed");

        assert!(left.is_empty());
        assert!(right.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn invalid_reading_returns_parse_failed() {
        let error = parse_batch_off_runtime(strings(&["10", "bad", "30"]))
            .await
            .expect_err("invalid readings must fail");

        assert_eq!(error, ProcessingError::ParseFailed);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn blocking_parser_panic_is_reported() {
        let error = parse_batch_off_runtime(strings(&["10", "panic", "30"]))
            .await
            .expect_err("blocking parser panic should be surfaced");

        assert_eq!(error, ProcessingError::WorkerPanicked);
    }
}
