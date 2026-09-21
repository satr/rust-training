use tokio::task::JoinHandle;

async fn collect_async_worker_values(
    worker_count: usize,
    values_per_worker: usize,
) -> Vec<usize> {
    todo!()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio tasks exercise");
}

#[cfg(test)]
mod tests {
    use super::collect_async_worker_values;

    #[tokio::test(flavor = "current_thread")]
    async fn collects_values_from_all_tasks() {
        let mut values = collect_async_worker_values(3, 2).await;
        values.sort();

        assert_eq!(values, vec![0, 0, 1, 1, 2, 2]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn zero_tasks_produce_no_values() {
        assert!(collect_async_worker_values(0, 10).await.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn tasks_can_return_zero_values() {
        assert!(collect_async_worker_values(3, 0).await.is_empty());
    }
}
