use std::sync::{Arc, Mutex};

fn increment_in_threads(thread_count: usize, increments_per_thread: usize) -> usize {
    let mut handles = Vec::new();
    let counter = Arc::new(Mutex::new(0usize));
    for _ in 0..thread_count {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let result = *counter.lock().unwrap();
    result
}

fn main() {
    println!("Arc Mutex threads exercise");
}

#[cfg(test)]
mod tests {
    use super::increment_in_threads;

    #[test]
    fn all_thread_increments_are_preserved() {
        assert_eq!(increment_in_threads(4, 25), 100);
    }

    #[test]
    fn zero_workers_produce_zero() {
        assert_eq!(increment_in_threads(0, 100), 0);
    }

    #[test]
    fn workers_with_no_increments_produce_zero() {
        assert_eq!(increment_in_threads(4, 0), 0);
    }
}
