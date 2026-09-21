use tokio::sync::oneshot;

async fn collect_until_cancelled(
    values: Vec<usize>,
    cancellation: oneshot::Receiver<()>,
) -> Vec<usize> {
    todo!()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio cancellation exercise");
}

#[cfg(test)]
mod tests {
    use super::collect_until_cancelled;
    use tokio::sync::oneshot;

    #[tokio::test(flavor = "current_thread")]
    async fn returns_all_values_when_not_cancelled() {
        let (_cancel_tx, cancel_rx) = oneshot::channel();

        assert_eq!(
            collect_until_cancelled(vec![10, 20, 30], cancel_rx).await,
            vec![10, 20, 30]
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn returns_no_values_when_cancelled_before_start() {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        cancel_tx.send(()).unwrap();

        assert!(collect_until_cancelled(vec![10, 20, 30], cancel_rx)
            .await
            .is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_work_returns_immediately() {
        let (_cancel_tx, cancel_rx) = oneshot::channel();

        assert!(collect_until_cancelled(Vec::new(), cancel_rx)
            .await
            .is_empty());
    }
}
