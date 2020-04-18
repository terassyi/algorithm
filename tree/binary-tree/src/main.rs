fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
enum BinaryTree<T> {
    Nil,
    Node(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

impl<T: PartialOrd+Clone+Copy> Clone for BinaryTree<T> {
    fn clone(&self) -> BinaryTree<T> {
        match *self {
            BinaryTree::Nil => BinaryTree::Nil,
            BinaryTree::Node(ref d, ref left, ref right) => {
                BinaryTree::Node(*d, (*left).clone(), (*right).clone())
            }
        }
    }
}

impl<T: PartialOrd+Clone+Copy> BinaryTree<T> {
    fn new(data: T) -> Self {
        BinaryTree::Node(data, Box::new(BinaryTree::Nil), Box::new(BinaryTree::Nil))
    }

    fn search(&self, data: &T) -> bool {
        match *self {
            BinaryTree::Nil => false,
            BinaryTree::Node(ref d, ref left, ref right) => {
                if data == d {
                    true
                } else if data < d {
                    left.search(&data)
                } else {
                    right.search(&data)
                }
            }
        }
    }

    fn insert(&mut self, data: T) -> BinaryTree<T> {
        match *self {
            BinaryTree::Nil => {
                BinaryTree::Node(data, Box::new(BinaryTree::Nil), Box::new(BinaryTree::Nil))
            },
            BinaryTree::Node(ref d, ref mut left, ref mut right) => {
                if *d > data {
                    BinaryTree::Node(*d, Box::new(left.insert(data)), (*right).clone())
                } else {
                    BinaryTree::Node(*d, (*left).clone(), Box::new(right.insert(data)))
                }
            }
        }
    }

    fn vec_to_binary_tree(data: &Vec<T>) -> BinaryTree<T> {
        let mut node = BinaryTree::Nil;
        for v in data.iter() {
            node = node.insert(*v);
        }
        node
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        assert_eq!(super::BinaryTree::Node(1 as usize, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil)), super::BinaryTree::<usize>::new(1 as usize));
    }

    #[test]
    fn test_insert() {
        let mut node = super::BinaryTree::Nil;
        let wanted = super::BinaryTree::Node(3,
            Box::new(super::BinaryTree::Node(2, Box::new(super::BinaryTree::Node(1, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil))), Box::new(super::BinaryTree::Nil))),
            Box::new(super::BinaryTree::Node(4, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil))),
        );
        assert_eq!(node.insert(3).insert(2).insert(4).insert(1), wanted);
    }

    #[test]
    fn test_search() {
        let node = super::BinaryTree::Node(3,
            Box::new(super::BinaryTree::Node(2, Box::new(super::BinaryTree::Node(1, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil))), Box::new(super::BinaryTree::Nil))),
            Box::new(super::BinaryTree::Node(4, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil))),
        ); 
        assert_eq!(node.search(&1), true);
    }

    #[test]
    fn test_vec_to_binary_tree() {
        let data = vec![3,2,4];
        let node = super::BinaryTree::vec_to_binary_tree(&data);

        let wanted = super::BinaryTree::Node(3, 
            Box::new(super::BinaryTree::Node(2, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil))),
            Box::new(super::BinaryTree::Node(4, Box::new(super::BinaryTree::Nil), Box::new(super::BinaryTree::Nil))),
        );

        assert_eq!(node, wanted);
    }
}

