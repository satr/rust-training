use std::sync::{Arc, Mutex};
use std::thread;

fn increment_in_threads(thread_count: usize, increments_per_thread: usize) -> usize {
    todo!()
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
