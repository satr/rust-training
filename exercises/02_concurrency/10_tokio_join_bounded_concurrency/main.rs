use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Debug, PartialEq, Eq)]
enum ProcessingError {
    InvalidConcurrencyLimit,
    WorkerPanicked,
}

async fn async_double(value: usize) -> usize {
    tokio::task::yield_now().await;
    value * 2
}

async fn process_batch_bounded(
    values: Vec<usize>,
    max_in_flight: usize,
) -> Result<Vec<usize>, ProcessingError> {
    if max_in_flight == 0 {
        return Err(ProcessingError::InvalidConcurrencyLimit);
    }
    let mut handles = Vec::new();
    let permits = Arc::new(Semaphore::new(max_in_flight));
    for value in values {
        let task_permits = Arc::clone(&permits);
        handles.push(tokio::spawn(async move {
            let _permit = task_permits.acquire_owned().await.unwrap();
            async_double(value).await
        }));
    }
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.map_err(|_| ProcessingError::WorkerPanicked)?;
        results.push(result);
    }
    Ok(results)
}

async fn process_two_batches(
    left: Vec<usize>,
    right: Vec<usize>,
    max_in_flight: usize,
) -> Result<(Vec<usize>, Vec<usize>), ProcessingError> {
    let (result_left, result_right) = tokio::join!(
        process_batch_bounded(left, max_in_flight),
        process_batch_bounded(right, max_in_flight)
    );
    Ok((result_left?, result_right?))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio join + bounded concurrency exercise");
}

#[cfg(test)]
mod tests {
    use super::{process_two_batches, ProcessingError};

    #[tokio::test(flavor = "current_thread")]
    async fn processes_both_batches_concurrently_with_join() {
        let (mut left, mut right) = process_two_batches(vec![1, 2, 3], vec![4, 5], 2)
            .await
            .expect("batch processing should succeed");

        left.sort_unstable();
        right.sort_unstable();

        assert_eq!(left, vec![2, 4, 6]);
        assert_eq!(right, vec![8, 10]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_batches_return_empty_vectors() {
        let (left, right) = process_two_batches(Vec::new(), Vec::new(), 3)
            .await
            .expect("empty input should still succeed");

        assert!(left.is_empty());
        assert!(right.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn rejects_zero_concurrency_limit() {
        let error = process_two_batches(vec![1, 2], vec![3, 4], 0)
            .await
            .expect_err("zero concurrency must be rejected");

        assert_eq!(error, ProcessingError::InvalidConcurrencyLimit);
    }
}
