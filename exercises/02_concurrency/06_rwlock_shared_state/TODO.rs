use std::sync::{Arc, RwLock};
use std::thread;

fn snapshot_sum_after_updates(initial: Vec<i32>, updates: &[i32], reader_count: usize) -> Vec<i32> {
    todo!()
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
