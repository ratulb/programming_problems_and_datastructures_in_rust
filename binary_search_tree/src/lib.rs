use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
#[derive(Debug, Default)]
//Default is for the case when we delete a node. We get the value out of the deleted node by
//pushing a default value into it.

struct Node<T: Ord + Default + Clone + std::fmt::Debug> {
    key: T,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Ord + Default + Clone + std::fmt::Debug> Node<T> {
    //New up a bare node
    fn new(value: T) -> Self {
        Self {
            key: value,
            left: None,
            right: None,
            parent: None,
        }
    }
    //Create a new node wrapped in a RefCell which in turn is
    //wrapped in a Rc
    fn wrapped_node(value: T) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Node::new(value))))
    }

    //Get a reference to `Node` key
    fn key(&self) -> &T {
        &self.key
    }
    //Does this node has a left child tree
    fn has_left(&self) -> bool {
        self.left.is_some()
    }

    //Does this Tree rooted at this has a right child tree
    fn has_right(&self) -> bool {
        self.right.is_some()
    }
    //Get a shared handle to the root of left child tree
    fn left_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.left
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(Rc::clone))
    }
    //Is the given key has the same value as the left tree root node
    //Used when deleting nodes from the tree
    fn is_left_child(&self, key: &T) -> bool {
        Self::left_node(self)
            .as_ref()
            .map(|node| node.borrow().key() == key)
            .unwrap_or(false)
    }

    //Get a shared handle to the right tree's root node
    fn right_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.right
            .as_ref()
            .and_then(|tree| tree.borrow().0.as_ref().map(Rc::clone))
    }

    //Node's parent is a weak reference that we initialize when the node
    //is inserted to the tree - we need to upgrade it to a strong reference
    //to get to the underlying parent node
    fn upgrade_parent(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }

    //Replace this node's key with the value might be there inside the input
    //Used during delete. If this node is being deleted, then this node's key
    //is flushed out with minimum node's value that is on the right side of
    //this node
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

    //Delete a node with single child or no child but node being deleted has parent
    //left: bool -> Should we delete left or right child?
    fn delete_child(&mut self, left: bool) -> Option<T> {
        //First take out the left or right child based on the flag passed in
        let deleted = match left {
            true => self.left.take(),
            false => self.right.take(),
        };
        //result is tuple of the form as shown below
        //result = (deleted.key, deleted.parent, left or right child of deleted)
        let result = match deleted
            .and_then(|tree| tree.take().0)
            .map(|node| node.take())
            .map(|node| (node.key, node.parent, node.left.or(node.right)))
        {
            //Set deleted node's left or right child's parent to the parent of deleted
            Some((key, parent, mut tree)) => {
                if let Some(ref mut inner) = tree {
                    if let Some(ref mut tree_node) = inner.borrow_mut().0 {
                        tree_node.borrow_mut().parent = parent;
                    }
                }
                //(deleted.key, left or right of deleted
                (Some(key), tree)
            }
            None => (None, None),
        };
        //Set self left right to deleted left or right
        match left {
            true => self.left = result.1,
            false => self.right = result.1,
        }
        //deleted key
        result.0
    }
    //Delete an node when it has two children
    fn delete(mut target: Option<Rc<RefCell<Node<T>>>>) -> Option<T> {
        let min = target
            .as_ref()
            .and_then(|target| target.borrow().right_node().as_ref().and_then(Tree::min));
        let min_parent = min.as_ref().and_then(|min| min.borrow().upgrade_parent());
        let right_parent = Node::right_parent(target.as_ref(), min_parent.as_ref());
        let left_or_right = right_parent.as_ref().and_then(|parent| {
            min.as_ref()
                .map(|min| parent.borrow().is_left_child(min.borrow().key()))
        });
        let min = match right_parent {
            None => None,
            Some(parent) => match left_or_right {
                None => None,
                Some(true) => parent.borrow_mut().left.take(),
                Some(false) => parent.borrow_mut().right.take(),
            },
        };
        let mut min = min.map(|tree| tree.take()).and_then(|tree| tree.root());
        let min_right = min
            .as_ref()
            .and_then(|min| min.borrow_mut().right.take())
            .and_then(|tree| tree.borrow().root());
        if let Some(ref min_right) = min_right {
            min_right.borrow_mut().parent = right_parent
                .as_ref()
                .map(|parent| Rc::downgrade(&Rc::clone(parent)));
        }
        if let Some(parent) = right_parent {
            match left_or_right {
                None => {}
                Some(true) => {
                    parent.borrow_mut().left = Tree::with_node(min_right.as_ref().cloned())
                }
                Some(false) => {
                    parent.borrow_mut().right = Tree::with_node(min_right.as_ref().cloned())
                }
            }
        }
        match target {
            Some(ref mut target) => target
                .borrow_mut()
                .replace_key(min.take().map(|min| min.take().key)),
            None => None,
        }
    }
}
#[derive(Debug, Default)]
pub struct Tree<T: Ord + Default + Clone + std::fmt::Debug>(Option<Rc<RefCell<Node<T>>>>);

impl<T: Ord + Default + Clone + std::fmt::Debug> Tree<T> {
    //Initialize a new tree with the value
    pub fn new(value: T) -> Self {
        Tree(Some(Rc::new(RefCell::new(Node::new(value)))))
    }
    //Create new tree rooted at the input node
    fn new_branch(node: Node<T>) -> Option<Rc<RefCell<Tree<T>>>> {
        Some(Rc::new(RefCell::new(Tree(Some(Rc::new(RefCell::new(
            node,
        )))))))
    }

    fn with_node(node: Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Tree<T>>>> {
        match node {
            None => None,
            node @ Some(_) => Some(Rc::new(RefCell::new(Tree(node)))),
        }
    }

    //Get a shared handle to the root of the tree
    fn root(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.0.as_ref().map(Rc::clone)
    }

    //Find the min - given a node. Result could be given node itself
    //if no more left branch is there
    fn min(node: &Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        match node.borrow().left_node() {
            Some(ref left_node) => Self::min(left_node),
            None => Some(Rc::clone(node)),
        }
    }

    //Populate tree with a new key
    //Duplicate keys are not added
    //Recursively calls itself to find place to add the supplied key
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

    //Find the node containing the supplied key reference
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
            Some(_) => None, //Make the compiler happy
            None => None,
        }
    }

    //Delete a node with key that equals the supplied key
    //Returns the deleted key as Some(key) or None otherwise
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
                            //Has left child - remove left child's parent ref and set it as
                            //tree root
                            (false, true, false, false) => {
                                let root = self.root().take();
                                self.0 = root.as_ref().and_then(|root| {
                                    root.borrow().left_node().map(|node| {
                                        node.borrow_mut().parent.take();
                                        node
                                    })
                                });
                                //Return root's key
                                root.map(|root| root.take().key)
                            }
                            //Has right child - remove right child's parent ref and set it as
                            //tree root
                            (false, false, true, false) => {
                                let root = self.root().take();
                                self.0 = root.as_ref().and_then(|root| {
                                    root.borrow().right_node().map(|node| {
                                        node.borrow_mut().parent.take();
                                        node
                                    })
                                });
                                //Return root's key
                                root.map(|root| root.take().key)
                            }
                            //Has got both children - delegate to Node::delete
                            (false, true, true, true) => {
                                let result = Node::delete(target);
                                result
                            }
                            (_, _, _, _) => None,
                        }
                    }
                    //target node being deleted has got a parent
                    true => match (no_child, has_left, has_right, has_both) {
                        (true, false, false, false)
                        | (false, true, false, false)
                        | (false, false, true, false) => {
                            let parent = node.borrow().upgrade_parent();
                            //Is it left or right? If no child then left is false
                            let left = parent
                                .as_ref()
                                .map_or(false, |parent| parent.borrow().is_left_child(key));
                            //Delefate to node.delete_child with boolean flag left
                            parent.and_then(|parent| parent.borrow_mut().delete_child(left))
                        }
                        //Has parent, has two children
                        //Delegate to Node::delete
                        (false, true, true, true) => Node::delete(target),
                        (_, _, _, _) => None,
                    },
                }
            }
        }
    }
    //Returns the minimum key value (if any) or `None` otherwise
    //Delegates to internal min function
    pub fn minimum(&self) -> Option<T> {
        let node = self.root();
        match node {
            None => None,
            Some(ref inner) => Self::min(inner).map(|n| n.borrow().key.clone()),
        }
    }
    //Does a key exists in the tree?
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

    //Does this contains the other tree?
    pub fn contains(&self, other: &Self) -> bool {
        match self {
            Tree(None) => match other {
                Tree(_) => false,
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

    //Is this tree is identical to other tree?
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

    //Find the height of the tree
    pub fn height(&self) -> usize {
        let root = self.root();
        match root {
            None => 0,
            Some(ref node)
                if node.borrow().left_node().is_none() & node.borrow().right_node().is_none() =>
            {
                1
            }
            Some(ref node) => {
                let left_tree_height = node
                    .borrow()
                    .left
                    .as_ref()
                    .map(|tree| Self::height(&tree.borrow()))
                    .unwrap_or(0);
                let right_tree_height = node
                    .borrow()
                    .right
                    .as_ref()
                    .map(|tree| Self::height(&tree.borrow()))
                    .unwrap_or(0);
                1 + std::cmp::max(left_tree_height, right_tree_height)
            }
        }
    }
    //Return the lowest common ancestor for two given keys
    pub fn lowest_common_ancestor(&self, this: &T, that: &T) -> Option<T> {
        if let Some(ref root) = self.root() {
            let root = root.borrow();
            if root.key() < this && root.key() < that {
                if let Some(ref right) = root.right {
                    return Self::lowest_common_ancestor(&right.borrow(), this, that);
                } else {
                    return None;
                }
            } else if root.key() > this && root.key() > that {
                if let Some(ref left) = root.left {
                    return Self::lowest_common_ancestor(&left.borrow(), this, that);
                } else {
                    return None;
                }
            } else {
                return Some(root.key().clone());
            }
        } else {
            None
        }
    }

    //Find nth smallest in the binary seach tree
    pub fn nth_smallest(&self, nth: usize) -> Option<T> {
        let mut current_pos = 0;
        let mut result = None;
        Self::nth_smallest_helper(self.root(), &mut current_pos, nth, &mut result);
        result
    }

    fn nth_smallest_helper(
        node: Option<Rc<RefCell<Node<T>>>>,
        current_pos: &mut usize,
        nth: usize,
        result: &mut Option<T>,
    ) {
        if let Some(inner) = node {
            Self::nth_smallest_helper(inner.borrow().left_node(), current_pos, nth, result);
            *current_pos += 1;
            if *current_pos == nth {
                *result = Some(inner.borrow().key().clone());
            }
            Self::nth_smallest_helper(inner.borrow().right_node(), current_pos, nth, result);
        }
    }
    //kth smallest element - iterative
    pub fn kth_smallest(&self, k: usize) -> Option<T> {
        let mut curr = self.root();
        let mut stack = Vec::new();
        let mut n = 0;
        while curr.is_some() || !stack.is_empty() {
            while curr.is_some() {
                stack.push(curr.as_ref().cloned());
                curr = curr.and_then(|curr| curr.borrow().left_node());
            }
            curr = stack.pop().flatten();
            n += 1;
            if n == k {
                return curr.map(|curr| curr.borrow().key().clone());
            }
            curr = curr.and_then(|curr| curr.borrow().right_node());
        }
        None
    }

    //Validate the if the tree is valid
    //We use an interative process here
    pub fn validate(&self) -> bool {
        match self.0 {
            None => true,
            Some(ref root) => {
                let mut current = Some(Rc::clone(root));
                let mut stack = Vec::new();
                let mut previous = None;
                let mut crossed_root = false;
                let mut first = Some(false);
                while current.is_some() || !stack.is_empty() {
                    while current.is_some() {
                        stack.push(current.as_ref().cloned());
                        current = current.and_then(|inner| inner.borrow().left_node());
                    }
                    current = stack.pop().and_then(|popped| popped);
                    //Has moved to the right side of the tree
                    if !crossed_root {
                        crossed_root = current.as_ref().map(|curr| Rc::ptr_eq(curr, root)).unwrap();
                        if let Some(first) = first.as_mut() {
                            *first = crossed_root;
                        }
                    }
                    match previous {
                        None => previous = current.as_ref().map(Rc::clone),
                        Some(ref prev) => match current.as_ref().map(|curr| {
                            (
                                crossed_root,
                                &mut first,
                                curr.borrow().key() > prev.borrow().key(),
                                curr.borrow().key() < root.borrow().key(),
                            )
                        }) {
                            Some((false, Some(false), true, true)) => {
                                previous = current.as_ref().map(Rc::clone)
                            }
                            Some((true, Some(true), true, false)) => {
                                if let Some(first) = first.as_mut() {
                                    *first = false;
                                }
                                previous = current.as_ref().map(Rc::clone)
                            }
                            Some((true, Some(false), true, false)) => {
                                previous = current.as_ref().map(Rc::clone)
                            }
                            _ => return false,
                        },
                    }
                    current = current.and_then(|inner| inner.borrow().right_node());
                }
                true
            }
        }
    }

    //Create a height balanced tree from a sorted array
    //The array passed in gets mutated - its elements are replaced with default
    //values for type `T`
    pub fn from_sorted_array(array: &mut [T]) -> Option<Self> {
        fn wrap_tree<T: Ord + Default + Clone + std::fmt::Debug>(
            tree: Option<Tree<T>>,
        ) -> Option<Rc<RefCell<Tree<T>>>> {
            match tree {
                None => None,
                tree @ Some(_) => tree.map(|tree| Rc::new(RefCell::new(tree))),
            }
        }
        fn from_array<T: Ord + Default + Clone + std::fmt::Debug>(
            array: &mut [T],
            left: i32,
            right: i32,
        ) -> Option<Tree<T>> {
            if left > right {
                return None;
            } else {
                let mid = left + (right - left) / 2;
                let tree = Tree::new(std::mem::take(&mut array[mid as usize]));
                let right = from_array(array, mid + 1, right);
                let left = from_array(array, left, mid - 1);
                let mut right = wrap_tree(right);
                let mut left = wrap_tree(left);
                if let Some(ref mut root) = tree.root() {
                    root.borrow_mut().left = left.as_ref().cloned();
                    root.borrow_mut().right = right.as_ref().cloned();
                }
                if let Some(ref mut left) = left {
                    if let Some(ref mut tree_node) = left.borrow_mut().root() {
                        tree_node.borrow_mut().parent = tree
                            .root()
                            .as_ref()
                            .map(|root| Rc::downgrade(&Rc::clone(root)));
                    }
                }
                if let Some(ref mut right) = right {
                    if let Some(ref mut tree_node) = right.borrow_mut().root() {
                        tree_node.borrow_mut().parent = tree
                            .root()
                            .as_ref()
                            .map(|root| Rc::downgrade(&Rc::clone(root)));
                    }
                }
                return Some(tree);
            }
        }
        from_array(array, 0, (array.len() - 1) as i32)
    }

    //Update a node key in the tree
    pub fn update(&mut self, key: &T, new_val: T) -> bool {
        let mut node = self.node_iter().find(|node| node.borrow().key() == key);
        match node {
            None => false,
            Some(ref mut inner) => {
                inner.borrow_mut().replace_key(Some(new_val));
                true
            }
        }
    }

    //Get an iterator for the tree's keys
    //Remember - calling iter on the tree would not consume the tree
    //iterator.next would return Option<T>
    //T is cloned
    //Keys would would be returned level wise
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.root().map(|node| {
                let mut next = VecDeque::new();
                next.push_front(node);
                next
            }),
        }
    }

    //Returns an iterator that consumes the tree elements one by one
    //when calling next on it
    //Root of the tree is eviced when next is called on the iterator
    pub fn into_iter(&mut self) -> IntoIter<'_, T> {
        IntoIter {
            tree: match self {
                Tree(None) => None,
                Tree(_) => Some(self),
            },
        }
    }
    //Returns an internal iterator that lets update values of the tree
    fn node_iter(&mut self) -> NodeIter<T> {
        NodeIter {
            next: self.root().map(|node| {
                let mut next = VecDeque::new();
                next.push_front(node);
                next
            }),
        }
    }
}

#[derive(Debug)]
pub struct Iter<T: Ord + Default + Clone + std::fmt::Debug> {
    next: Option<VecDeque<Rc<RefCell<Node<T>>>>>,
}

impl<T: Ord + Default + Clone + std::fmt::Debug> Iterator for Iter<T> {
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
pub struct IntoIter<'a, T: Ord + Default + Clone + std::fmt::Debug> {
    tree: Option<&'a mut Tree<T>>,
}

impl<T: Ord + Default + Clone + std::fmt::Debug> Iterator for IntoIter<'_, T> {
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
#[derive(Debug)]
struct NodeIter<T: Ord + Default + Clone + std::fmt::Debug> {
    next: Option<VecDeque<Rc<RefCell<Node<T>>>>>,
}
impl<T: Ord + Default + Clone + std::fmt::Debug> Iterator for NodeIter<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(ref mut queue) => {
                let popped = queue.pop_back();
                match popped {
                    None => None,
                    Some(ref inner) => {
                        let node = inner.borrow();
                        if let Some(ref left) = node.left_node() {
                            queue.push_front(Rc::clone(left));
                        }
                        if let Some(ref right) = node.right_node() {
                            queue.push_front(Rc::clone(right));
                        }
                        Some(Rc::clone(inner))
                    }
                }
            }
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
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(25));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_delete1() {
        let mut tree = Tree::new(25);

        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(5);
        let node15 = tree.root().and_then(|root| {
            root.borrow()
                .left_node()
                .and_then(|left| left.borrow().right_node())
        });
        let deleted = Node::delete(node15);
        assert_eq!(deleted, Some(15));
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
        let result = tree.delete(&15);
        assert_eq!(result, Some(15));
        let result = tree.delete(&25);
        assert_eq!(result, Some(25));
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
        let result = tree.delete(&21);
        assert_eq!(result, Some(21));
        let result = tree.delete(&27);
        assert_eq!(result, Some(27));
    }

    #[test]
    fn test_height() {
        let mut tree = Tree::new(1);
        assert_eq!(tree.height(), 1);
        tree.delete(&1);
        assert_eq!(tree.height(), 0);
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.height(), 3);
        tree.insert(-1);
        tree.insert(-2);
        assert_eq!(tree.height(), 3);
        tree.insert(-3);
        assert_eq!(tree.height(), 4);
    }
    #[test]
    fn test_lowest_common_ancestor() {
        let mut tree = Tree::new(6);
        tree.insert(2);
        tree.insert(8);
        tree.insert(0);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);
        tree.insert(3);
        tree.insert(5);
        assert_eq!(tree.lowest_common_ancestor(&3, &5), Some(4));
        assert_eq!(tree.lowest_common_ancestor(&2, &5), Some(2));
        assert_eq!(tree.lowest_common_ancestor(&0, &5), Some(2));
        assert_eq!(tree.lowest_common_ancestor(&3, &7), Some(6));
    }
    #[test]
    fn test_nth_smallest() {
        let mut tree = Tree::new(6);
        tree.insert(2);
        tree.insert(8);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);
        tree.insert(3);
        tree.insert(5);
        assert_eq!(tree.nth_smallest(1), Some(1));
        assert_eq!(tree.nth_smallest(2), Some(2));
        assert_eq!(tree.nth_smallest(4), Some(4));
        assert_eq!(tree.nth_smallest(8), Some(8));
        assert_eq!(tree.nth_smallest(9), Some(9));
    }
    #[test]
    fn test_kth_smallest() {
        let mut tree = Tree::new(6);
        tree.insert(2);
        tree.insert(8);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);
        tree.insert(3);
        tree.insert(5);
        assert_eq!(tree.kth_smallest(1), Some(1));
        assert_eq!(tree.kth_smallest(2), Some(2));
        assert_eq!(tree.kth_smallest(4), Some(4));
        assert_eq!(tree.kth_smallest(8), Some(8));
        assert_eq!(tree.kth_smallest(9), Some(9));
    }
    #[test]
    fn test_tree_is_valid() {
        let mut tree = Tree::new(6);
        tree.insert(2);
        tree.insert(8);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);
        tree.insert(3);
        tree.insert(5);
        assert!(tree.validate());
    }
    #[test]
    fn test_tree_validate() {
        let mut tree = Tree::new(6);
        tree.insert(2);
        tree.insert(8);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);
        tree.insert(3);
        tree.insert(5);
        assert!(tree.validate());
    }
    #[test]
    fn test_from_sorted_array() {
        let mut array = [1, 2, 3];
        let tree = Tree::from_sorted_array(&mut array);
        assert_eq!(array, [0, 0, 0]);
        let mut tree = tree.unwrap();
        assert!(tree.validate());
        assert_eq!(tree.height(), 2);
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let tree = Tree::from_sorted_array(&mut array);
        let mut tree = tree.unwrap();
        assert!(tree.validate());
        assert_eq!(tree.height(), 4);
        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_update_tree() {
        let mut array = [1, 2, 3];
        let tree = Tree::from_sorted_array(&mut array);
        let mut tree = tree.unwrap_or_default();
        assert!(tree.validate());
        tree.update(&1, 200);
        assert!(!tree.validate());
        let mut array = [1, 2, 3, 4, 50, 60, 70, 80, 90];
        let tree = Tree::from_sorted_array(&mut array);
        let mut tree = tree.unwrap_or_default();
        assert!(tree.validate());
        tree.update(&4, 100);
        assert!(!tree.validate());
    }
}
