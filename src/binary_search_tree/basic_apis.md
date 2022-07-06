# Basic APIs

### Following are some of the basic APIs in the `Node` construct to start with:
```rust, ignore
    fn new(value: T) -> Self {
        Self {
            key: value,
            left: None,
            right: None,
            parent: None,
        }
    }
```
The API above is internal and used to new up a bare bone `Node` struct with a value of type T.
```rust, ignore
   fn wrapped_node(value: T) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Node::new(value))))
   }
```
We need our `Node` to be wrapped in `RefCell` and `Rc`. Above is a helper that avoids code
repetition.

### Following are some of additional helper APIs that are used internally: 

```rust, ignore
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
    //Clone this parent which is a weak reference
    //Used during deletion of a node. When the minimum node is taken
    //out from the right side of the node being deleted, the minimum node's
    //right tree(if any) - has to be hoisted up to point to the minimum
    //node's parent
    fn parent(&self) -> Option<Weak<RefCell<Node<T>>>> {
        self.parent.as_ref().map(Weak::clone)
    }

```

### Following sre some helper API's for the `Tree` struct:

```rust, ignore
   //Initialize a new tree with the value
    pub fn new(value: T) -> Self {
        Tree(Some(Rc::new(RefCell::new(Node::new(value)))))
    }
    
   //Get a shared handle to the root of the tree
    fn root(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.0.as_ref().map(Rc::clone)
    }

   //Create new tree rooted at the input node
    fn new_branch(node: Node<T>) -> Option<Rc<RefCell<Tree<T>>>> {
        Some(Rc::new(RefCell::new(Tree(Some(Rc::new(RefCell::new(
            node,
        )))))))
    }

   //Find the min - given a node. Result could be given node itself
   //if no more left branch is there
    fn min(node: &Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        match node.borrow().left_node() {
            Some(ref left_node) => Self::min(left_node),
            None => Some(Rc::clone(node)),
        }
    }
```

Now that helper APIs are out of the way - next we will look at how to populate the tree.
