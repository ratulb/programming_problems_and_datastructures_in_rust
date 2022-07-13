# Tree update

We can update the value of any node in the binary search tree. We search the key to be 
updated by using `Iterator` find method any mutate its value.

### Tree update implementation
```rust, ignore
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
```
