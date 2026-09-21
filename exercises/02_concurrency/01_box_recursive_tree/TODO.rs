#[derive(Debug, PartialEq)]
enum Tree {
    Leaf(i32),
    Branch {
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    fn sum(&self) -> i32 {
        todo!()
    }

    fn leaf_count(&self) -> usize {
        todo!()
    }

    fn depth(&self) -> usize {
        todo!()
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
