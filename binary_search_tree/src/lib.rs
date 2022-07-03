use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
#[derive(Debug, Default)]
//Default is for the case when we delete a node. We get the value out of the deleted node by
//pushing a default value into it.

struct Node<T: Clone + Ord + Default + std::fmt::Debug> {
    key: T,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Clone + Ord + Default + std::fmt::Debug> Node<T> {
    fn key(&self) -> &T {
        &self.key
    }
    fn has_left(&self) -> bool {
        self.left.is_some()
    }
    fn has_right(&self) -> bool {
        self.right.is_some()
    }

    fn left_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.left
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(Rc::clone))
    }

    fn is_left_child(&self, key: &T) -> bool {
        self.left
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(|n| n.borrow().key() == key))
            .unwrap_or(false)
    }

    fn right_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.right
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(Rc::clone))
    }

    fn upgradded_parent(&self) -> Option<Rc<RefCell<Node<T>>>> {
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
            (Some(this_one), Some(that_one)) => match Rc::ptr_eq(this_one, that_one) {
                true => this,
                false => that,
            },
        }
    }

    fn parent(&self) -> Option<Weak<RefCell<Node<T>>>> {
        self.parent.as_ref().map(Weak::clone)
    }
}

#[derive(Debug, Default)]
pub struct Tree<T: Clone + Ord + Default + std::fmt::Debug>(Option<Rc<RefCell<Node<T>>>>);

impl<T: Clone + Ord + Default + std::fmt::Debug> Tree<T> {
    pub fn new(value: T) -> Self {
        Tree(Some(Rc::new(RefCell::new(Node::new(value)))))
    }

    fn new_branch(node: Node<T>) -> Option<Rc<RefCell<Tree<T>>>> {
        Some(Rc::new(RefCell::new(Tree(Some(Rc::new(RefCell::new(
            node,
        )))))))
    }

    pub fn insert(&mut self, value: T) {
        match self.0 {
            Some(ref mut curr_tree_root) => {
                let mut node = curr_tree_root.borrow_mut();
                if node.key > value {
                    match node.left {
                        Some(ref mut tree) => Self::insert(&mut tree.borrow_mut(), value),
                        None => {
                            let parent = Some(Rc::downgrade(&Rc::clone(curr_tree_root)));
                            let mut left = Node::new(value);
                            left.parent = parent;
                            node.left = Tree::new_branch(left);
                        }
                    }
                } else if node.key < value {
                    match node.right {
                        Some(ref mut tree) => Self::insert(&mut tree.borrow_mut(), value),
                        None => {
                            let parent = Some(Rc::downgrade(&Rc::clone(curr_tree_root)));
                            let mut right = Node::new(value);
                            right.parent = parent;
                            node.right = Tree::new_branch(right);
                        }
                    }
                }
            }
            None => self.0 = Node::wrapped_node(value),
        }
    }

    fn find(&self, key: &T) -> Option<Rc<RefCell<Node<T>>>> {
        match self.0 {
            Some(ref node) if node.borrow().key() == key => Some(Rc::clone(node)),
            Some(ref node) if node.borrow().key() > key => match node.borrow().left {
                Some(ref left) => Self::find(&left.borrow(), key),
                None => None,
            },
            Some(ref node) if node.borrow().key() < key => match node.borrow().right {
                Some(ref right) => Self::find(&right.borrow(), key),
                None => None,
            },
            Some(_) => None, //Make the comiler happy
            None => None,
        }
    }

    fn root(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.0.as_ref().map(Rc::clone)
    }

    pub fn delete(&mut self, key: &T) -> Option<T> {
        let target = Self::find(self, key);
        match target {
            None => None,
            Some(ref node) => {
                let has_left = node.borrow().has_left();
                let has_right = node.borrow().has_right();

                let has_both = has_left && has_right;
                let no_child = !has_left && !has_right;
                let has_parent = node.borrow().parent.is_some();
                match has_parent {
                    false => {
                        //Delete root - root has no parent ref - hence differential treatment
                        match (no_child, has_left, has_right, has_both) {
                            (true, false, false, false) => {
                                self.0.take().map(|root| root.take().key)
                            }

                            (false, true, false, false) => self.0.take().map(|root| {
                                let mut node = root.borrow_mut();
                                node.left
                                    .as_ref()
                                    .map(|inner_tree| match inner_tree.take() {
                                        Tree(None) => self.0 = None,
                                        Tree(left_tree) => {
                                            self.0 = left_tree.map(|inner| {
                                                inner.borrow_mut().parent.take();
                                                inner
                                            })
                                        }
                                    });
                                std::mem::take(&mut node.key)
                            }),

                            (false, false, true, false) => self.0.take().map(|root| {
                                let mut node = root.borrow_mut();
                                node.right
                                    .as_ref()
                                    .map(|inner_tree| match inner_tree.take() {
                                        Tree(None) => self.0 = None,
                                        Tree(right_tree) => {
                                            self.0 = right_tree.map(|inner| {
                                                //Take out the out going parent ref
                                                inner.borrow_mut().parent.take();
                                                inner
                                            })
                                        }
                                    });
                                std::mem::take(&mut node.key)
                            }),
                            (false, true, true, true) => Node::delete_right_min(target),
                            (_, _, _, _) => None,
                        }
                    }

                    true => match (no_child, has_left, has_right, has_both) {
                        (true, false, false, false)
                        | (false, true, false, false)
                        | (false, false, true, false) => {
                            let parent = node.borrow().upgradded_parent();
                            let left = parent
                                .as_ref()
                                .map_or(false, |parent| parent.borrow().is_left_child(key));
                            parent.and_then(|parent| parent.borrow_mut().delete_child(left))
                        }
                        (false, true, true, true) => Node::delete_right_min(target),
                        (_, _, _, _) => None,
                    },
                }
            }
        }
    }

    fn min(node: &Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        match node.borrow().left_node() {
            Some(ref left_node) => Self::min(left_node),
            None => Some(Rc::clone(node)),
        }
    }

    pub fn minimum(&self) -> Option<T> {
        let node = self.root();
        match node {
            None => None,
            Some(ref inner) => Self::min(inner).map(|n| n.borrow().key.clone()),
        }
    }

    pub fn exists(&self, key: &T) -> bool {
        match self.0 {
            Some(ref node) => {
                node.borrow().key() == key || {
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

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.root().map(|node| {
                let mut next = VecDeque::new();
                next.push_front(node);
                next
            }),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            tree: match self {
                Tree(None) => None,
                Tree(_) => Some(self),
            },
        }
    }
}
#[derive(Debug)]
pub struct Iter<T: Clone + Ord + Default + std::fmt::Debug> {
    next: Option<VecDeque<Rc<RefCell<Node<T>>>>>,
}

impl<T: Clone + Ord + Default + std::fmt::Debug> Iterator for Iter<T> {
    type Item = T;
    //Level wise iterator
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(ref mut queue) => {
                let popped = queue.pop_back();
                match popped {
                    None => None,
                    Some(ref node) => {
                        let node = node.borrow();
                        if let Some(ref left) = node.left_node() {
                            queue.push_front(Rc::clone(left));
                        }
                        if let Some(ref right) = node.right_node() {
                            queue.push_front(Rc::clone(right));
                        }
                        Some(node.key.clone())
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T: Clone + Ord + Default + std::fmt::Debug> {
    tree: Option<&'a mut Tree<T>>,
}

impl<T: Clone + Ord + Default + std::fmt::Debug> Iterator for IterMut<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.tree {
            None => None,
            Some(ref mut tree) => match tree.0 {
                None => None,
                Some(ref mut node) => {
                    let key = node.borrow().key.clone();
                    tree.delete(&key);
                    Some(key)
                }
            },
        }
    }
}

impl<T: Clone + Ord + Default + std::fmt::Debug> Node<T> {
    fn new(value: T) -> Self {
        Self {
            key: value,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn wrapped_node(value: T) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Node::new(value))))
    }

    //Delete a node with single child
    fn delete_child(&mut self, left: bool) -> Option<T> {
        let deleted = match left {
            true => self.left.take(),
            false => self.right.take(),
        };

        let result = match deleted
            .and_then(|tree| tree.take().0)
            .map(|node| node.take())
            .map(|node| (node.key, node.parent, node.left.or(node.right)))
        {
            Some((key, parent, mut tree)) => {
                if let Some(ref mut inner) = tree {
                    if let Some(ref mut tree_node) = inner.borrow_mut().0 {
                        tree_node.borrow_mut().parent = parent;
                    }
                }
                (Some(key), tree)
            }
            None => (None, None),
        };
        match left {
            true => self.left = result.1,
            false => self.right = result.1,
        }
        result.0
    }

    fn evict_left(&mut self) -> Option<T> {
        self.left
            .take() //Default is replacing node in 'take' - No issues - left is wiped out anyway
            .and_then(|tree| {
                //Left child to be deleted is inner cell
                tree.borrow().0.as_ref().map(|node| {
                    //Rc<RefCell>
                    let mut node = node.take(); //Min node, Rc.take
                                                //Take out the right tree if any under the min node being evicted
                                                //Link it to the parent of the min node being deleted and place it
                                                //in self.left place because self.left is being evicted
                    let right_maybe_tree = node.right.take(); //Right child tree if any
                    self.left = right_maybe_tree
                        .map(|right_tree| {
                            //Tree
                            if let Some(ref mut right_tree_node) =
                                right_tree.borrow_mut().0.as_mut()
                            {
                                //Right tree node now points to grandpa now
                                right_tree_node.borrow_mut().parent = node.parent();
                            }
                            right_tree
                        })
                        .take();
                    node.key
                })
            })
    }

    fn delete_right_min(mut node: Option<Rc<RefCell<Node<T>>>>) -> Option<T> {
        let min = node
            .as_ref()
            .and_then(|node| node.borrow().right_node().as_ref().and_then(Tree::min));
        let min_parent = min.as_ref().and_then(|min| min.borrow().upgradded_parent());
        let mut min_right_child = min.as_ref().and_then(|min| {
            min.borrow_mut()
                .right
                .take()
                .as_ref()
                .and_then(|child| child.borrow().root())
        });
        if let Some(ref mut child_node) = min_right_child {
            child_node.borrow_mut().parent = min
                .as_ref()
                .and_then(|min| min.borrow().parent().as_ref().map(|parent| parent.clone()));
        }
        let mut right_parent = Node::right_parent(node.as_ref(), min_parent.as_ref());
        if let Some(ref mut parent) = right_parent {
            parent.borrow_mut().right =
                min_right_child.map(|right_child| Rc::new(RefCell::new(Tree(Some(right_child)))));
        }
        match node {
            Some(ref mut node) => node.borrow_mut().replace_key(min.map(|min| min.take().key)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        tree.insert(200);
        assert!(tree.find(&200).is_some());
        assert!(tree.find(&42).is_some());
        assert!(tree.find(&24).is_some());
        assert!(tree.find(&40).is_some());
        assert!(tree.find(&35).is_some());
        assert!(tree.find(&100).is_none());
    }

    #[test]
    fn test_min() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        tree.insert(5);
        assert_eq!(Tree::min(&tree.0.unwrap()).unwrap().take().key, 5);
    }

    #[test]
    fn test_delete_child() {
        let mut tree = Tree::new(42);
        tree.insert(24);
        tree.insert(40);
        tree.insert(35);
        tree.insert(50);
        assert!(tree.find(&24).is_some());
        tree.0.as_mut().unwrap().borrow_mut().delete_child(true);
        assert!(tree.find(&24).is_none());
    }

    #[test]
    fn test_delete() {
        let mut tree = Tree::new(42);
        let result = tree.delete(&42);
        assert!(tree.find(&42).is_none());
        assert_eq!(result, Some(42));
        //Left only tree
        let mut tree = Tree::new(3);
        tree.insert(2);
        tree.insert(1);
        let result = tree.delete(&3);
        assert!(tree.find(&3).is_none());
        assert_eq!(result, Some(3));
        let result = tree.delete(&2);
        assert_eq!(result, Some(2));
        //Right only tree
        let mut tree = Tree::new(1);
        tree.insert(2);
        tree.insert(3);
        let result = tree.delete(&1);
        assert!(tree.find(&1).is_none());
        assert_eq!(result, Some(1));
        let result = tree.delete(&2);
        assert_eq!(result, Some(2));
    }
    #[test]
    fn delete_root_with_both_subtrees() {
        //Right and left tree - evict root
        let mut tree = Tree::new(20);
        tree.insert(10);
        tree.insert(30);
        tree.insert(25);
        let result = tree.delete(&20);
        assert_eq!(result, Some(20));
    }

    #[test]
    fn delete_root_with_both_subtree_1_level() {
        //Right and left tree - evict root
        let mut tree = Tree::new(20);
        tree.insert(10);
        tree.insert(30);
        let result = tree.delete(&20);
        assert_eq!(result, Some(20));
    }
    #[test]
    fn delete_node_with_parent_no_child() {
        //Right and left tree - evict root
        let mut tree = Tree::new(20);
        tree.insert(10);
        tree.insert(30);
        let result = tree.delete(&10);
        assert_eq!(result, Some(10));
        assert!(tree.find(&10).is_none());

        let result = tree.delete(&30);
        assert_eq!(result, Some(30));
        assert!(tree.find(&30).is_none());
    }

    #[test]
    fn delete_node_with_parent_one_child() {
        //Right and left tree - evict root
        let mut tree = Tree::new(25);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(30);
        let result = tree.delete(&10);
        assert_eq!(result, Some(10));
        assert!(tree.find(&10).is_none());

        assert!(tree.find(&30).is_some());
        let result = tree.delete(&30);
        assert_eq!(result, Some(30));
        assert!(tree.find(&30).is_none());

        let result = tree.delete(&25);
        assert_eq!(result, Some(25));
        assert!(tree.find(&25).is_none());
        let result = tree.delete(&20);
        assert_eq!(result, Some(20));
        assert!(tree.find(&20).is_none());
        let result = tree.delete(&15);
        assert_eq!(result, Some(15));
        assert!(tree.find(&15).is_none());
        assert!(tree.0.is_none())
    }

    #[test]
    fn delete_node_with_parent_both_childrent() {
        let mut tree = Tree::new(25);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        let result = tree.delete(&10);
        assert_eq!(result, Some(10));
        assert!(tree.find(&10).is_none());

        let result = tree.delete(&25);
        assert_eq!(result, Some(25));
        assert!(tree.find(&25).is_none());

        let result = tree.delete(&20);
        assert_eq!(result, Some(20));
        assert!(tree.find(&20).is_none());

        let result = tree.delete(&5);
        assert_eq!(result, Some(5));
        assert!(tree.find(&5).is_none());

        let result = tree.delete(&15);
        assert_eq!(result, Some(15));
        assert!(tree.find(&15).is_none());

        let result = tree.delete(&15);
        assert_eq!(result, None);
        assert!(tree.find(&15).is_none());
    }

    #[test]
    fn minimum_test() {
        let mut tree = Tree::new(25);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        assert_eq!(tree.minimum(), Some(5));
        let _ = tree.delete(&5);
        assert_eq!(tree.minimum(), Some(10));
        let _ = tree.delete(&10);
        assert_eq!(tree.minimum(), Some(15));
        let _ = tree.delete(&15);
        assert_eq!(tree.minimum(), Some(20));
        let _ = tree.delete(&20);
        assert_eq!(tree.minimum(), Some(25));
        let _ = tree.delete(&25);
        assert_eq!(tree.minimum(), None);
    }

    #[test]
    fn iter_test() {
        let mut tree = Tree::new(25);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(25));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn itermut_test() {
        let mut tree = Tree::new(25);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        let mut iter = tree.iter_mut();
        assert_eq!(iter.next(), Some(25));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_delete_right_min() {
        let mut tree = Tree::new(25);

        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        let node10 = tree.root().and_then(|root| {
            root.borrow()
                .left_node()
                .and_then(|left| left.borrow().right_node())
        });

        println!("Target = {:?}", node10);
        let deleted = Node::delete_right_min(node10);
        println!("Deleted = {:?}", deleted);
        println!("Tree = {:?}", tree);
    }

    #[test]
    fn test_delete_2() {
        let mut tree = Tree::new(25);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        let result = tree.delete(&10);
        assert_eq!(result, Some(10));
        println!("Tree = {:?}", tree);
        let result = tree.delete(&15);
        assert_eq!(result, Some(15));
        println!("Tree = {:?}", tree);
        let result = tree.delete(&25);
        assert_eq!(result, Some(25));
        println!("Tree = {:?}", tree);
    }

    #[test]
    fn test_delete_3() {
        let mut tree = Tree::new(27);
        tree.insert(18);
        tree.insert(24);
        tree.insert(21);
        tree.insert(25);
        tree.insert(30);
        let result = tree.delete(&24);
        assert_eq!(result, Some(24));
        println!("Tree = {:?}", tree);
        let result = tree.delete(&21);
        assert_eq!(result, Some(21));
        println!("Tree = {:?}", tree);
        let result = tree.delete(&27);
        assert_eq!(result, Some(27));
        println!("Tree = {:?}", tree);
    }
}
