use std::sync::mpsc;
use std::thread;

fn collect_worker_values(worker_count: usize, values_per_worker: usize) -> Vec<usize> {
    todo!()
}

fn main() {
    println!("threads and channels exercise");
}

#[cfg(test)]
mod tests {
    use super::collect_worker_values;

    #[test]
    fn collects_values_from_all_workers() {
        let mut values = collect_worker_values(3, 2);
        values.sort();

        assert_eq!(values, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn zero_workers_produce_no_values() {
        assert!(collect_worker_values(0, 10).is_empty());
    }

    #[test]
    fn workers_can_send_zero_values() {
        assert!(collect_worker_values(3, 0).is_empty());
    }
}
