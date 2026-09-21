use tokio::sync::mpsc;

async fn collect_async_worker_values(worker_count: usize, values_per_worker: usize) -> Vec<usize> {
    todo!()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio channels exercise");
}

#[cfg(test)]
mod tests {
    use super::collect_async_worker_values;

    #[tokio::test(flavor = "current_thread")]
    async fn collects_values_sent_by_all_workers() {
        let mut values = collect_async_worker_values(3, 2).await;
        values.sort_unstable();

        assert_eq!(values, vec![0, 0, 1, 1, 2, 2]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn zero_workers_return_an_empty_vector() {
        assert!(collect_async_worker_values(0, 4).await.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn zero_values_per_worker_return_an_empty_vector() {
        assert!(collect_async_worker_values(3, 0).await.is_empty());
    }
}
