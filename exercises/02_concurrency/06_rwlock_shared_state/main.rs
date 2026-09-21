use std::sync::{Arc, RwLock};
use std::thread;

fn snapshot_sum_after_updates(initial: Vec<i32>, updates: &[i32], reader_count: usize) -> Vec<i32> {
    let values = Arc::new(RwLock::new(initial));
    let mut reader_handles = Vec::new();
    let writer_values = Arc::clone(&values);
    let writer_updates = updates.to_vec();
    let writer_handle = thread::spawn(move || {
        let mut writer_guard = writer_values.write().unwrap();
        for val in writer_updates {
            writer_guard.push(val);
        }
    });
    writer_handle.join().unwrap();
    for _ in 0..reader_count {
        let thread_values = Arc::clone(&values);
        let reader_handle = thread::spawn(move || {
            let reader_guard = thread_values.read().unwrap();
            let sum = reader_guard.iter().sum::<i32>();
            sum
        });
        reader_handles.push(reader_handle);
    }
    let mut sums = Vec::new();
    for handle in reader_handles {
        let sum = handle.join().unwrap();
        sums.push(sum);
    }
    sums
}

fn main() {
    println!("RwLock shared state exercise");
}

#[cfg(test)]
mod tests {
    use super::snapshot_sum_after_updates;

    #[test]
    fn readers_see_the_updated_state() {
        let snapshots = snapshot_sum_after_updates(vec![10, 20], &[5, 15], 3);

        assert_eq!(snapshots, vec![50, 50, 50]);
    }

    #[test]
    fn no_readers_produce_no_snapshots() {
        assert!(snapshot_sum_after_updates(vec![1, 2], &[3], 0).is_empty());
    }

    #[test]
    fn an_empty_state_can_be_updated() {
        assert_eq!(snapshot_sum_after_updates(Vec::new(), &[7, 8], 1), vec![15]);
    }
}
