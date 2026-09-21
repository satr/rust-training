use tokio::sync::watch;

#[derive(Debug, PartialEq, Eq)]
enum ShutdownError {
    InvalidWorkerCount,
    ShutdownRequested,
    WorkerPanicked,
}

async fn process_until_shutdown(
    values: Vec<usize>,
    worker_count: usize,
    shutdown: watch::Receiver<bool>,
) -> Result<Vec<usize>, ShutdownError> {
    if worker_count == 0 {
        return Err(ShutdownError::InvalidWorkerCount);
    }

    let _ = values;
    let _ = shutdown;
    todo!()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio watch shutdown exercise");
}

#[cfg(test)]
mod tests {
    use super::{process_until_shutdown, ShutdownError};
    use tokio::sync::watch;

    #[tokio::test(flavor = "current_thread")]
    async fn processes_all_values_when_shutdown_is_not_requested() {
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);

        let result = process_until_shutdown(vec![1, 2, 3, 4], 2, shutdown_rx)
            .await
            .expect("processing should succeed");

        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn reports_shutdown_requested() {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        shutdown_tx.send(true).unwrap();

        let error = process_until_shutdown(vec![1, 2, 3], 2, shutdown_rx)
            .await
            .expect_err("shutdown should be reported");

        assert_eq!(error, ShutdownError::ShutdownRequested);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn rejects_zero_workers() {
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);

        let error = process_until_shutdown(vec![1, 2], 0, shutdown_rx)
            .await
            .expect_err("zero workers should be rejected");

        assert_eq!(error, ShutdownError::InvalidWorkerCount);
    }
}
