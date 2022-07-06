# Tree minimum

The `minimum` pub API returns the minimum entry in the tree if the tree is not empty. It call anoth-ther internal API called `min`. The `min` function accepts a reference to `Rc<RefCell<Node<T>>>` 
and returns `OptionRc<RefCell<Node<T>>>>`. Its does so by going to the left of the passed in node 
reference as far as possible or returns current left's root node otherwise.

The tree `minimum` just calls the internal `min` function passing its root node's reference and
returns either returned `Node`'s (if found) key or `None` otherwise.

### Below are the public `minimum` and internal `min` APIs.

```rust, ignore

    //Returns the minimum key value (if any) or `None` otherwise
    //Delegates to internal min function
    pub fn minimum(&self) -> Option<T> {
        let node = self.root();
        match node {
            None => None,
            Some(ref inner) => Self::min(inner).map(|n| n.borrow().key.clone()),
        }
    }
```
> **Note**: This is one place where we are using clone on T i.e. `key.clone()` above.

```rust, ignore

    //Find the min - given a node. Result could be given node itself
    //if no more left branch is there
    fn min(node: &Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        match node.borrow().left_node() {
            Some(ref left_node) => Self::min(left_node),
            None => Some(Rc::clone(node)),
        }
    }

```
