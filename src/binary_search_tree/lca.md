# Lowest common ancestor (LCA)

Here we present how to find the lowest common ancestor for two given keys. LCA is the 
immediate parent that the given keys share. Or the parent could be one the keys - if that 
key happens to the parent of the other given key.

The LCA would be in the left or right side of a node - if the node's key is bigger or 
smaller than both the given keys. If that condition trips - that would mean given keys 
are on the either side of the node or one of the keys is the node itself and the other 
would be beneath it.

### Following is the LCA implentation:
```rust, ignore
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
```  
