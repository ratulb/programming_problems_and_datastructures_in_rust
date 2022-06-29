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

impl<T: Ord + Default + std::fmt::Debug + Clone> Node<T> {
    fn key(&self) -> &T {
        &self.key
    }
    fn has_left(&self) -> bool {
        self.left.is_some()
    }
    fn has_right(&self) -> bool {
        self.right.is_some()
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    fn left_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.left
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(Rc::clone))
    }

    fn right_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.right
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(Rc::clone))
    }

    fn upgraded_parent(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }

    fn replace_key(&mut self, key: Option<T>) -> Option<T> {
        key.map(|k| std::mem::replace(&mut self.key, k))
    }

    //To avoid already borrowed error - if Rc<RefCell> pointing to same location
    fn right_parent<'a>(
        this: Option<&'a Rc<RefCell<Node<T>>>>,
        that: Option<&'a Rc<RefCell<Node<T>>>>,
    ) -> Option<&'a Rc<RefCell<Node<T>>>> {
        match (this, that) {
            (None, None) => None,
            (Some(_), None) => this,
            (None, Some(_)) => that,
            (Some(ref this_one), Some(ref that_one)) => {
                if Rc::ptr_eq(this_one, that_one) {
                    this
                } else {
                    that
                }
            }
        }
    }

    fn parent(&self) -> Option<Weak<RefCell<Node<T>>>> {
        self.parent.as_ref().map(Weak::clone)
    }
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

    fn find(&self, key: &T) -> Option<Rc<RefCell<Node<T>>>> {
        match self.root() {
            Some(ref node) if node.borrow().key() == key => Some(Rc::clone(node)),
            Some(ref node) => match node.borrow().left {
                Some(ref left) => Self::find(&left.borrow(), key),
                None => match node.borrow().right {
                    Some(ref right) => Self::find(&right.borrow(), key),
                    None => None,
                },
            },
            None => None,
        }
    }

    fn root(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.0.as_ref().map(Rc::clone)
    }

    fn left_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        Self::root(self).and_then(|root| root.borrow().left_node())
    }

    fn right_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        Self::root(self).and_then(|root| root.borrow().right_node())
    }

    pub fn delete(tree: &mut Tree<T>, key: &T) -> Option<T> {
        let mut being_deleted = Self::find(tree, key);
        match being_deleted {
            None => None,
            Some(ref node) => {
                let has_left = node.borrow().has_left();
                let has_right = node.borrow().has_right();

                let has_both = has_left && has_right;
                let no_child = !has_left && !has_right;
                let mut parent = node.borrow().upgraded_parent();
                match parent {
                    None => {
                        //Delete root
                        match (no_child, has_left, has_right, has_both) {
                            (true, false, false, false) => {
                                tree.0.take().map(|root| root.take().key)
                            }

                            (false, true, false, false) => tree.0.take().map(|root| {
                                let mut node = root.borrow_mut();
                                node.left
                                    .as_ref()
                                    .map(|inner_tree| match inner_tree.take() {
                                        Tree(None) => tree.0 = None,
                                        Tree(mut left_tree) => {
                                            tree.0 = left_tree.map(|inner| {
                                                inner.borrow_mut().parent.take();
                                                inner
                                            })
                                        }
                                    });
                                std::mem::take(&mut node.key)
                            }),

                            (false, false, true, false) => tree.0.take().map(|root| {
                                let mut node = root.borrow_mut();
                                node.right
                                    .as_ref()
                                    .map(|inner_tree| match inner_tree.take() {
                                        Tree(None) => tree.0 = None,
                                        Tree(mut right_tree) => {
                                            tree.0 = right_tree.map(|inner| {
                                                inner.borrow_mut().parent.take();
                                                inner
                                            })
                                        }
                                    });
                                std::mem::take(&mut node.key)
                            }),
                            (false, true, true, true) => {
                                let root = tree.root();
                                let parent = root.as_ref().and_then(|root| {
                                    root.borrow()
                                        .right_node()
                                        .as_ref()
                                        .and_then(Self::find_min)
                                        .and_then(|min| min.borrow().upgraded_parent())
                                });

                                let right_parent =
                                    Node::right_parent(root.as_ref(), parent.as_ref());
                                let evicted =
                                    right_parent.and_then(|rp| rp.borrow_mut().evict_left());
                                root.and_then(|r| r.borrow_mut().replace_key(evicted))
                            }
                            (_, _, _, _) => None,
                        }
                    }

                    Some(ref mut parent) => {
                        match (no_child, has_left, has_right, has_both) {
                            (true, false, false, false) => return None,
                            (false, true, false, false) => return None,
                            (false, false, true, false) => return None,
                            (false, true, true, true) => return None,
                            (_, _, _, _) => return None,
                        };
                        let mut min_on_left = Self::find_min(node);

                        match min_on_left {
                            None => None,
                            Some(ref n) => {
                                let mut parent = n.borrow().upgraded_parent();
                                None
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_min(node: &Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        match node.borrow().left {
            Some(ref tree) => match tree.borrow().0 {
                Some(ref inner_node) => Self::find_min(inner_node),
                None => Some(Rc::clone(node)),
            },
            None => Some(Rc::clone(node)),
        }
    }
    pub fn exists(&self, key: &T) -> bool {
        match self.0 {
            Some(ref node) => {
                node.borrow().key == *key || {
                    let in_left = match node.borrow().left {
                        Some(ref tree) => Self::exists(&tree.borrow(), key),
                        None => false,
                    };

                    let in_right = match node.borrow().right {
                        Some(ref tree) => Self::exists(&tree.borrow(), key),
                        None => false,
                    };
                    in_left || in_right
                }
            }
            None => false,
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
                    if Self::is_identical(self, that) {
                        return true;
                    }
                    let left_contains = match this.borrow().left {
                        Some(ref tree) => Self::contains(&tree.borrow(), that),
                        None => false,
                    };
                    let right_contains = match this.borrow().right {
                        Some(ref tree) => Self::contains(&tree.borrow(), that),
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

    pub fn remove_min(&mut self) -> Option<T> {
        match Self::parent_of_min(self) {
            Some(ref mut parent) => {
                let mut parent = parent.borrow_mut();
                parent.evict_left()
            }
            None => match self.0 {
                None => None,
                Some(ref mut node) => Self::remove_root_one_child(self, false),
            },
        }
    }
    //Remove root node with only left or right child
    fn remove_root_one_child(tree: &mut Tree<T>, left: bool) -> Option<T> {
        tree.0.take().map(|root| {
            let mut node = root.borrow_mut();
            let left_or_right = if left { &node.left } else { &node.right };
            left_or_right.as_ref().map(|tree_| match tree_.take() {
                Tree(None) => tree.0 = None,
                Tree(left_or_right_tree) => tree.0 = left_or_right_tree,
            });
            std::mem::take(&mut node.key)
        })
    }

    fn parent_of_min(&self) -> Option<Rc<RefCell<Node<T>>>> {
        match self.0 {
            Some(ref cell) => match cell.borrow().left {
                Some(ref left) => Self::parent_of_min(&left.borrow()),
                None => cell.borrow().upgraded_parent(),
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
    fn delete_child(&mut self, left: bool) {
        let mut child = if left {
            self.left.take()
        } else {
            self.right.take()
        };

        if let Some(ref mut child) = child {
            if let Some(ref node) = child.borrow().0 {
                let mut deleted = node.borrow_mut();
                let (lp, rp) = match deleted.parent {
                    None => (None, None),
                    Some(ref p) => (Some(p.clone()), Some(p.clone())),
                };
                if let Some(ref mut left_node) = deleted.left {
                    if let Some(ref mut child_node) = left_node.borrow_mut().0 {
                        child_node.borrow_mut().parent = lp;
                    }
                    self.left = Some(std::mem::take(left_node));
                }
                if let Some(ref mut right_node) = deleted.right {
                    if let Some(ref mut child_node) = right_node.borrow_mut().0 {
                        child_node.borrow_mut().parent = rp;
                    }
                    self.right = Some(std::mem::take(right_node));
                }
            }
        }
    }

    fn evict_left(&mut self) -> Option<T> {
        self.left
            .take() //Default is replacing inner in 'take' - No issues - left is wiped out anyway
            .and_then(|cell| {
                //Left child(child to be deleted) inner cell
                cell.borrow().0.as_ref().map(|inner| {
                    let mut left_node = inner.take(); //Min node
                    let right_child_tree = left_node.right.take();
                    self.left = right_child_tree
                        .map(|right_cell| {
                            if let Some(ref mut right_child_node) =
                                right_cell.borrow_mut().0.as_mut()
                            {
                                right_child_node.borrow_mut().parent = left_node.parent();
                            }
                            right_cell
                        })
                        .take();
                    left_node.key
                })
            })
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

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.remove_min(), Some(1));
        assert_eq!(tree.remove_min(), Some(2));
        assert_eq!(tree.remove_min(), Some(3));
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

    #[test]
    fn test_exists() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        assert!(tree.exists(&42));
        assert!(tree.exists(&24));
        assert!(tree.exists(&40));
        assert!(tree.exists(&35));
        assert!(!tree.exists(&100));
    }

    #[test]
    fn test_find() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        assert!(Tree::find(&tree, &42).is_some());
        assert!(Tree::find(&tree, &24).is_some());
        assert!(Tree::find(&tree, &40).is_some());
        assert!(Tree::find(&tree, &35).is_some());
        assert!(Tree::find(&tree, &100).is_none());
    }

    #[test]
    fn test_find_min() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        tree.insert(5);
        assert_eq!(Tree::find_min(&tree.0.unwrap()).unwrap().take().key, 5);
    }

    #[test]
    fn test_delete_child() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        tree.insert(50);
        assert!(Tree::find(&tree, &24).is_some());
        tree.0.as_mut().unwrap().borrow_mut().delete_child(true);
        assert!(Tree::find(&tree, &24).is_none());
    }

    #[test]
    fn test_delete() {
        let mut tree = Tree::new(42);
        println!("{:?}", tree);
        let result = Tree::delete(&mut tree, &42);
        println!("Tree now {:?}", tree);
        assert!(Tree::find(&tree, &42).is_none());
        assert_eq!(result, Some(42));
        //Left only tree
        let mut tree = Tree::new(3);
        tree.insert(2);
        tree.insert(1);
        println!("{:?}", tree);
        let result = Tree::delete(&mut tree, &3);
        println!("{:?}", tree);
        assert!(Tree::find(&tree, &3).is_none());
        assert_eq!(result, Some(3));
        let result = Tree::delete(&mut tree, &2);
        assert_eq!(result, Some(2));
        println!("{:?}", tree);
        //Right only tree
        let mut tree = Tree::new(1);
        tree.insert(2);
        tree.insert(3);
        println!("{:?}", tree);
        let result = Tree::delete(&mut tree, &1);
        println!("{:?}", tree);
        assert!(Tree::find(&tree, &1).is_none());
        assert_eq!(result, Some(1));
        let result = Tree::delete(&mut tree, &2);
        assert_eq!(result, Some(2));
        println!("{:?}", tree);
    }
    #[test]
    fn delete_root_with_both_subtrees() {
        //Right and left tree - evict root
        let mut tree = Tree::new(20);
        tree.insert(10);
        tree.insert(30);
        tree.insert(25);
        println!("{:?}", tree);
        let result = Tree::delete(&mut tree, &20);
        assert_eq!(result, Some(20));
        println!("Here it is ************{:?}", tree);
    }

    #[test]
    fn delete_root_with_both_subtree_1_level() {
        //Right and left tree - evict root
        let mut tree = Tree::new(20);
        tree.insert(10);
        tree.insert(30);
        println!("{:?}", tree);
        let result = Tree::delete(&mut tree, &20);
        assert_eq!(result, Some(20));
        println!("Here it is ************{:?}", tree);
    }
}
