use std::sync::mpsc;
use std::thread;

fn collect_worker_values(worker_count: usize, values_per_worker: usize) -> Vec<usize> {
    let mut handles = Vec::new();
    let (sender, receiver) = mpsc::channel();
    for worker_index in 0..worker_count {
        let worker_sender = sender.clone();
        let handle = thread::spawn(move || {
            for _ in 0..values_per_worker {
                worker_sender.send(worker_index).unwrap();
            }
        });
        handles.push(handle);
    }
    drop(sender);
    let mut values = Vec::new();
    for value in receiver {
        values.push(value);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    values
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
