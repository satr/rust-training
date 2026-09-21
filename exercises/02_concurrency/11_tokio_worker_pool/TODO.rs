use tokio::sync::{mpsc, oneshot};

#[derive(Debug, PartialEq, Eq)]
enum WorkerPoolError {
    InvalidWorkerCount,
    Cancelled,
    WorkerPanicked,
}

async fn process_with_workers(
    values: Vec<usize>,
    worker_count: usize,
    cancellation: oneshot::Receiver<()>,
) -> Result<Vec<usize>, WorkerPoolError> {
    if worker_count == 0 {
        return Err(WorkerPoolError::InvalidWorkerCount);
    }

    let _ = values;
    let _ = cancellation;
    todo!()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio worker pool exercise");
}

#[cfg(test)]
mod tests {
    use super::{process_with_workers, WorkerPoolError};
    use tokio::sync::oneshot;

    #[tokio::test(flavor = "current_thread")]
    async fn processes_values_in_input_order() {
        let (_cancel_tx, cancel_rx) = oneshot::channel();

        let result = process_with_workers(vec![1, 2, 3, 4, 5], 2, cancel_rx)
            .await
            .expect("worker pool should succeed");

        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn cancellation_is_reported() {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        cancel_tx.send(()).unwrap();

        let error = process_with_workers(vec![1, 2, 3], 2, cancel_rx)
            .await
            .expect_err("cancelled work should return an error");

        assert_eq!(error, WorkerPoolError::Cancelled);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn rejects_zero_workers() {
        let (_cancel_tx, cancel_rx) = oneshot::channel();

        let error = process_with_workers(vec![1, 2], 0, cancel_rx)
            .await
            .expect_err("zero workers should be rejected");

        assert_eq!(error, WorkerPoolError::InvalidWorkerCount);
    }
}
