# Tree exists

This API return `true` or `false` based on whether a `Node` corresponding to a input key reference
exists or not in the tree. It does so by recursively checking left or/and the right tree for the
presence of the key.

### Following is the function definition:

```rust, ignore
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
```
