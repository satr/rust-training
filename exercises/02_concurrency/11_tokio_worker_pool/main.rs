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
    mut cancellation: oneshot::Receiver<()>,
) -> Result<Vec<usize>, WorkerPoolError> {
    if worker_count == 0 {
        return Err(WorkerPoolError::InvalidWorkerCount);
    }

    let value_count = values.len();
    let (result_tx, mut result_rx) = mpsc::channel(value_count.max(1));
    let mut job_senders = Vec::with_capacity(worker_count);
    let mut handles = Vec::with_capacity(worker_count);

    for _ in 0..worker_count {
        let (job_tx, mut job_rx) = mpsc::channel(1);
        let worker_result_tx = result_tx.clone();

        job_senders.push(job_tx);
        handles.push(tokio::spawn(async move {
            while let Some((index, value)) = job_rx.recv().await {
                if worker_result_tx.send((index, value * 2)).await.is_err() {
                    break;
                }
            }
        }));
    }
    drop(result_tx);

    let mut cancelled = false;

    for (index, value) in values.into_iter().enumerate() {
        let worker = index % worker_count;
        tokio::select! {
            biased;

            _ = &mut cancellation => {
                cancelled = true;
                break;
            }
            send_result = job_senders[worker].send((index, value)) => {
                if send_result.is_err() {
                    return Err(WorkerPoolError::WorkerPanicked);
                }
            }
        }
    }

    drop(job_senders);

    if cancelled {
        abort_workers(handles).await;
        return Err(WorkerPoolError::Cancelled);
    }

    let mut results = vec![0; value_count];
    let mut cancelled_while_collecting = false;
    for _ in 0..value_count {
        tokio::select! {
            biased;

            _ = &mut cancellation => {
                cancelled_while_collecting = true;
                break;
            }
            result = result_rx.recv() => {
                let Some((index, value)) = result else {
                    return Err(WorkerPoolError::WorkerPanicked);
                };
                results[index] = value;
            }
        }
    }

    if cancelled_while_collecting {
        abort_workers(handles).await;
        return Err(WorkerPoolError::Cancelled);
    }

    for handle in handles {
        if handle.await.is_err() {
            return Err(WorkerPoolError::WorkerPanicked);
        }
    }

    Ok(results)
}

async fn abort_workers(handles: Vec<tokio::task::JoinHandle<()>>) {
    for handle in &handles {
        handle.abort();
    }

    for handle in handles {
        let _ = handle.await;
    }
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
