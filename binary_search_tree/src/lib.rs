use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
                parent.evict_min()
            }
            None => None,
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

#[derive(Debug, Default)]
struct Node<T: Ord + Default + std::fmt::Debug + Clone> {
    key: T,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
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
    
    fn evict_min(&mut self) -> Option<T> {
        self.left
            .take() //Default is replacing inner in 'take' - No issues left is wiped out anyway
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
    fn test_binary_search_tree() {
        let mut tree = Tree::new(6);

        tree.insert(5);
        tree.insert(-4);
        tree.insert(-3);
        tree.insert(4);
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        println!("Tree = {:?}", tree);
        //println!("Min = {:?}", tree.min());
        //let tree1 = Tree::new(None::<i32>);
        //println!("Min = {:?}", tree1.min());
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        println!("Tree now = {:?}", tree);
        println!("Min now = {:?}", tree.min());
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        println!("Tree = {:?}", tree);

        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        println!("Tree = {:?}", tree);
        tree.insert(-100);
        let removed = tree.remove_min();
        println!("Removed = {:?}", removed);
        println!("Tree = {:?}", tree);
    }
}
