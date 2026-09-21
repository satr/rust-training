#[derive(Debug, PartialEq)]
enum Tree {
    Leaf(i32),
    Branch { left: Box<Tree>, right: Box<Tree> },
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

    fn depth(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Branch { left, right } => 1 + left.depth().max(right.depth()),
        }
    }
}

fn main() {
    println!("Box recursive tree exercise");
}

#[cfg(test)]
mod tests {
    use super::Tree;

    fn sample_tree() -> Tree {
        Tree::Branch {
            left: Box::new(Tree::Leaf(10)),
            right: Box::new(Tree::Branch {
                left: Box::new(Tree::Leaf(20)),
                right: Box::new(Tree::Leaf(5)),
            }),
        }
    }

    #[test]
    fn sums_all_leaf_values() {
        assert_eq!(sample_tree().sum(), 35);
    }

    #[test]
    fn counts_leaves() {
        assert_eq!(sample_tree().leaf_count(), 3);
    }

    #[test]
    fn calculates_maximum_depth() {
        assert_eq!(sample_tree().depth(), 3);
        assert_eq!(Tree::Leaf(42).depth(), 1);
    }
}
