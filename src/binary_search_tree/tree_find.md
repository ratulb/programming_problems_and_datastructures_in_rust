# Tree find

The tree `find` function is used to find a `Node (Option<Rc<RefCell<Node<T>>>>)` corresponding to a
given key reference. It recursively visit tree's left or right branches - till it finds the node or
returns `None` otherwise.

### Following is the `find` function defintion:

```rust, ignore

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
```

Now with `find` function in place, next we will look at tree's `delete` function. But before that
we will look at `Node::delete` function because tree's `delete` is heavily dependent on `Node::delete`.
