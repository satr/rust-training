use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct ReadingStore {
    values: Vec<i32>,
}

type SharedStore = Rc<RefCell<ReadingStore>>;

fn add_reading(store: &SharedStore, value: i32) {
    todo!()
}

fn total(store: &SharedStore) -> i32 {
    todo!()
}

fn main() {
    println!("Rc RefCell shared store exercise");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::{add_reading, total, ReadingStore};

    #[test]
    fn multiple_handles_can_mutate_one_store() {
        let first_handle = Rc::new(RefCell::new(ReadingStore::default()));
        let second_handle = Rc::clone(&first_handle);

        add_reading(&first_handle, 10);
        add_reading(&second_handle, 20);

        assert_eq!(total(&first_handle), 30);
        assert_eq!(total(&second_handle), 30);
    }

    #[test]
    fn an_empty_store_has_zero_total() {
        let store = Rc::new(RefCell::new(ReadingStore::default()));

        assert_eq!(total(&store), 0);
    }
}
