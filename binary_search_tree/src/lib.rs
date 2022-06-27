use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Default)]
//Default is for the case when we delete a node. We get the value out of the deleted node by
//pushing a default value into it.

struct Node<T: Ord + Default + std::fmt::Debug + Clone> {
    key: T,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug, Default)]
pub struct Tree<T: Ord + Default + std::fmt::Debug + Clone>(Option<Rc<RefCell<Node<T>>>>);

impl<T: Ord + Default + std::fmt::Debug + Clone> Tree<T> {
    pub fn new(value: T) -> Self {
        Tree(Some(Rc::new(RefCell::new(Node::new(value)))))
    }
    pub fn insert(&mut self, value: T) {
        match self.0 {
            Some(ref mut cell) => {
                let mut node = cell.borrow_mut();
                if node.key > value {
                    match node.left {
                        Some(ref mut tree) => Self::insert(&mut tree.borrow_mut(), value),
                        None => {
                            let parent = Some(Rc::downgrade(&Rc::clone(cell)));
                            let mut left = Node::new(value);
                            left.parent = parent;
                            node.left = Some(Rc::new(RefCell::new(Tree(Some(Rc::new(
                                RefCell::new(left),
                            ))))));
                        }
                    }
                } else if node.key < value {
                    match node.right {
                        Some(ref mut tree) => Self::insert(&mut tree.borrow_mut(), value),
                        None => {
                            let parent = Some(Rc::downgrade(&Rc::clone(cell)));
                            let mut right = Node::new(value);
                            right.parent = parent;
                            node.right = Some(Rc::new(RefCell::new(Tree(Some(Rc::new(
                                RefCell::new(right),
                            ))))));
                        }
                    }
                }
            }
            None => self.0 = Some(Rc::new(RefCell::new(Node::new(value)))),
        }
    }

    pub fn min(&self) -> Option<T> {
        match self.0 {
            Some(ref cell) => match cell.borrow().left {
                Some(ref left) => Self::min(&left.borrow()),
                None => {
                    //We are cloning key  here
                    Some(cell.borrow().key.clone())
                }
            },
            None => None,
        }
    }

    pub fn in_order(&self) -> Vec<T> {
        let mut traversed = Vec::new();
        Self::inorder(self, &mut traversed);
        traversed
    }

    fn inorder(&self, traversed: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            if let Some(ref left) = node.borrow().left {
                Self::inorder(&left.borrow(), traversed);
            }
            traversed.push(node.borrow().key.clone());
            if let Some(ref right) = node.borrow().right {
                Self::inorder(&right.borrow(), traversed);
            }
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        match self {
            Tree(None) => match other {
                Tree(None) => false,
                Tree(Some(_)) => false,
            },
            Tree(Some(ref this)) => match other {
                Tree(None) => true,
                that @ Tree(_) => {
                    if Self::is_identical(self, &that) {
                        return true;
                    }
                    let left_contains = match this.borrow().left {
                        Some(ref tree) => Self::contains(&tree.borrow(), &that),
                        None => false,
                    };
                    let right_contains = match this.borrow().right {
                        Some(ref tree) => Self::contains(&tree.borrow(), &that),
                        None => false,
                    };
                    left_contains || right_contains
                }
            },
        }
    }

    pub fn is_identical(&self, other: &Self) -> bool {
        match self.0 {
            Some(ref this) => match other {
                Tree(Some(ref that)) => {
                    if this.borrow().key == that.borrow().key {
                        let this_left = &this.borrow().left;
                        let that_left = &that.borrow().left;
                        let this_right = &this.borrow().right;
                        let that_right = &that.borrow().right;
                        let left_matched = match this_left {
                            Some(ref this_tree) => match that_left {
                                Some(ref that_tree) => {
                                    return Self::is_identical(
                                        &this_tree.borrow(),
                                        &that_tree.borrow(),
                                    );
                                }
                                None => false,
                            },
                            None => that_left.is_none(),
                        };
                        let right_matched = match this_right {
                            Some(ref this_tree) => match that_right {
                                Some(ref that_tree) => {
                                    return Self::is_identical(
                                        &this_tree.borrow(),
                                        &that_tree.borrow(),
                                    );
                                }
                                None => false,
                            },
                            None => that_right.is_none(),
                        };
                        left_matched && right_matched
                    } else {
                        false
                    }
                }
                Tree(None) => false,
            },

            None => match other {
                Tree(Some(_)) => false,
                Tree(None) => true,
            },
        }
    }
    fn parent_of(node: &Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        node.borrow()
            .parent
            .as_ref()
            .map(|parent| parent.upgrade())
            .flatten()
    }

    pub fn remove_min(&mut self) -> Option<T> {
        match Self::parent_of_min(self) {
            Some(ref mut parent) => {
                let mut parent = parent.borrow_mut();
                parent.evict_left()
            }
            None => match self.0 {
                None => None,
                Some(_) => self.0.take().map(|root_cell| {
                    let mut root_node = root_cell.borrow_mut();
                    root_node.right.as_ref().map(|tree| match tree.take() {
                        Tree(None) => self.0 = None,
                        Tree(right_tree_node) => self.0 = right_tree_node,
                    });
                    std::mem::take(&mut root_node.key)
                }),
            },
        }
    }

    fn parent_of_min(&self) -> Option<Rc<RefCell<Node<T>>>> {
        match self.0 {
            Some(ref cell) => match cell.borrow().left {
                Some(ref left) => Self::parent_of_min(&left.borrow()),
                None => Self::parent_of(cell),
            },
            None => None,
        }
    }
}

impl<T: Ord + Default + std::fmt::Debug + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            key: value,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn evict_left(&mut self) -> Option<T> {
        self.left
            .take() //Default is replacing inner in 'take' - No issues - left is wiped out anyway
            .map(|cell| {
                //Left child(child to be deleted) inner cell
                cell.borrow().0.as_ref().map(|inner| {
                    let mut left_node = inner.take(); //Min node
                    let right_child_tree = left_node.right.take();
                    self.left = right_child_tree
                        .map(|right_cell| {
                            if let Some(ref mut right_child_node) =
                                right_cell.borrow_mut().0.as_mut()
                            {
                                right_child_node.borrow_mut().parent = match left_node.parent {
                                    Some(ref left_node_parent) => {
                                        Some(Weak::clone(left_node_parent))
                                    }
                                    None => None,
                                };
                            }
                            right_cell
                        })
                        .take();
                    left_node.key
                })
            })
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_search_tree_1() {
        let mut tree = Tree::new(None::<i32>);
        assert_eq!(tree.remove_min(), Some(None));
    }
    #[test]
    fn test_binary_search_tree_2() {
        let mut tree = Tree::new(42);
        assert_eq!(tree.remove_min(), Some(42));
        assert_eq!(tree.remove_min(), None);
    }
    #[test]
    fn test_binary_search_tree_3() {
        let mut tree = Tree::new(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.remove_min(), Some(1));
        assert_eq!(tree.remove_min(), Some(2));
        assert_eq!(tree.remove_min(), Some(3));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_4() {
        let mut tree = Tree::new(3);
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.remove_min(), Some(1));
        assert_eq!(tree.remove_min(), Some(2));
        assert_eq!(tree.remove_min(), Some(3));
        assert_eq!(tree.remove_min(), None);
    }
    #[test]
    fn test_binary_search_tree_5() {
        let mut tree = Tree::new(42);
        tree.insert(42);
        tree.insert(42);
        assert_eq!(tree.remove_min(), Some(42));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_6() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        assert_eq!(tree.remove_min(), Some(24));
        assert_eq!(tree.remove_min(), Some(42));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_7() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        assert_eq!(tree.remove_min(), Some(24));
        assert_eq!(tree.remove_min(), Some(35));
        assert_eq!(tree.remove_min(), Some(40));
        assert_eq!(tree.remove_min(), Some(42));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_8() {
        let mut tree = Tree::new(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        assert_eq!(tree.remove_min(), Some(1));
        assert_eq!(tree.remove_min(), Some(2));
        assert_eq!(tree.remove_min(), Some(3));
        assert_eq!(tree.remove_min(), Some(4));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_9() {
        let mut tree = Tree::new(10);
        tree.insert(20);
        tree.insert(15);
        tree.insert(18);
        assert_eq!(tree.min(), Some(10));
        assert_eq!(tree.remove_min(), Some(10));
        assert_eq!(tree.min(), Some(15));
        assert_eq!(tree.remove_min(), Some(15));
        assert_eq!(tree.remove_min(), Some(18));
        assert_eq!(tree.remove_min(), Some(20));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_10() {
        let mut tree = Tree::new(10);
        tree.insert(20);
        tree.insert(15);
        tree.insert(14);
        tree.insert(13);
        assert_eq!(tree.remove_min(), Some(10));
        assert_eq!(tree.min(), Some(13));
        assert_eq!(tree.remove_min(), Some(13));
        assert_eq!(tree.remove_min(), Some(14));
        assert_eq!(tree.remove_min(), Some(15));
        assert_eq!(tree.remove_min(), Some(20));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_binary_search_tree_11() {
        let mut tree = Tree::new(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.remove_min(), Some(1));
        assert_eq!(tree.remove_min(), Some(2));
        assert_eq!(tree.remove_min(), Some(3));
        tree.insert(1);
        assert_eq!(tree.remove_min(), Some(1));
        assert_eq!(tree.remove_min(), None);
    }

    #[test]
    fn test_identical() {
        let mut tree1 = Tree::new(1);
        tree1.insert(2);
        tree1.insert(3);
        assert!(!tree1.is_identical(&Tree(None)));
        let mut tree2 = Tree::new(1);
        assert!(!tree1.is_identical(&tree2));
        tree2.insert(2);
        tree2.insert(3);
        assert!(tree1.is_identical(&tree2));
        assert!(Tree::new(None::<String>).is_identical(&Tree::new(None)));
    }

    #[test]
    fn test_in_order() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        assert_eq!(tree.in_order(), vec![24, 35, 40, 42]);
    }

    #[test]
    fn test_contains() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        assert!(tree.contains(&Tree(None)));
        assert!(tree.contains(&Tree::new(35)));
        assert!(!tree.contains(&Tree::new(40)));

        let mut subtree = Tree::new(24);
        subtree.insert(40);
        subtree.insert(35);
        assert!(tree.contains(&subtree));
        assert!(!subtree.contains(&Tree::new(24)));
    }
}
