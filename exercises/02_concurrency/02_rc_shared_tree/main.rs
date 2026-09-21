use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum Tree {
    Leaf(i32),
    Branch { left: Rc<Tree>, right: Rc<Tree> },
}

impl Tree {
    fn sum(&self) -> i32 {
        match self {
            Tree::Leaf(value) => *value,
            Tree::Branch { left, right } => left.sum() + right.sum(),
        }
    }

    fn leaf_count(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Branch { left, right } => left.leaf_count() + right.leaf_count(),
        }
    }
}

fn main() {
    println!("Rc shared tree exercise");
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Tree;

    #[test]
    fn processes_a_shared_subtree_twice() {
        let shared = Rc::new(Tree::Leaf(7));
        let tree = Tree::Branch {
            left: Rc::clone(&shared),
            right: Rc::clone(&shared),
        };

        assert_eq!(tree.sum(), 14);
        assert_eq!(tree.leaf_count(), 2);
        assert_eq!(Rc::strong_count(&shared), 3);
    }

    #[test]
    fn cloning_rc_does_not_clone_the_tree() {
        let original = Rc::new(Tree::Leaf(42));
        let clone = Rc::clone(&original);

        assert_eq!(original, clone);
        assert_eq!(Rc::strong_count(&original), 2);
    }
}
