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
    mut shutdown: watch::Receiver<bool>,
) -> Result<Vec<usize>, ShutdownError> {
    if worker_count == 0 {
        return Err(ShutdownError::InvalidWorkerCount);
    }

    let (result_tx, mut result_rx) = tokio::sync::mpsc::channel(worker_count);

    let mut handles = Vec::with_capacity(worker_count);
    let mut job_senders = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let (job_sender_tx, mut job_sender_rx) = tokio::sync::mpsc::channel(1);
        job_senders.push(job_sender_tx);
        let mut worker_shutdown_rx = shutdown.clone();
        let worker_result_tx = result_tx.clone();
        handles.push(tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    shutdown_changed = worker_shutdown_rx.changed() => {
                        shutdown_changed.map_err(|_| ShutdownError::ShutdownRequested)?;
                        if *worker_shutdown_rx.borrow() {
                            return Err(ShutdownError::ShutdownRequested)
                        }
                    },
                    job = job_sender_rx.recv() => {
                        match job {
                            Some((index, value)) => {
                                if worker_result_tx.send((index, value * 2)).await.is_err() {
                                    return Err(ShutdownError::WorkerPanicked);
                                }
                            },
                            None => break,
                        }
                    },
                }
            }
            Ok(())
        }));
    }

    drop(result_tx);
    let values_len = values.len();
    let mut dispatch_error = None;

    for (index, value) in values.into_iter().enumerate() {
        let worker_index = index % worker_count;
        let send_result = tokio::select! {
            biased;
            shutdown_changed = shutdown.changed() => {
                match shutdown_changed {
                    Ok(()) if *shutdown.borrow() => Err(ShutdownError::ShutdownRequested),
                    Ok(()) => Ok(()),
                    Err(_) => Err(ShutdownError::ShutdownRequested),
                }
            }
            send_result = job_senders[worker_index].send((index, value)) => {
                send_result.map_err(|_| ShutdownError::WorkerPanicked)
            }
        };

        if let Err(error) = send_result {
            dispatch_error = Some(error);
            break;
        }
    }

    drop(job_senders);
    if let Some(error) = dispatch_error {
        return match finish_workers(handles).await {
            Ok(()) => Err(error),
            Err(cleanup_error) => Err(cleanup_error),
        };
    }

    let mut results = vec![0; values_len];
    let mut collection_error = None;
    for _ in 0..values_len {
        let receive_result = tokio::select! {
            biased;
            shutdown_changed = shutdown.changed() => {
                match shutdown_changed {
                    Ok(()) if *shutdown.borrow() => Err(ShutdownError::ShutdownRequested),
                    Ok(()) => Ok(None),
                    Err(_) => Err(ShutdownError::ShutdownRequested),
                }
            }
            result = result_rx.recv() => {
                Ok(result)
            }
        };

        match receive_result {
            Ok(Some((index, value))) => results[index] = value,
            Ok(None) => {
                collection_error = Some(if *shutdown.borrow() {
                    ShutdownError::ShutdownRequested
                } else {
                    ShutdownError::WorkerPanicked
                });
                break;
            }
            Err(error) => {
                collection_error = Some(error);
                break;
            }
        }
    }

    let worker_result = finish_workers(handles).await;
    if let Some(error) = collection_error {
        return Err(error);
    }
    worker_result?;
    Ok(results)
}

async fn finish_workers(
    handles: Vec<tokio::task::JoinHandle<Result<(), ShutdownError>>>,
) -> Result<(), ShutdownError> {
    let mut first_error = None;

    for handle in handles {
        let result = match handle.await {
            Ok(result) => result,
            Err(_) => Err(ShutdownError::WorkerPanicked),
        };

        if first_error.is_none() {
            first_error = result.err();
        }
    }

    match first_error {
        Some(error) => Err(error),
        None => Ok(()),
    }
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
