# Tree contains

The tree `contains` API tells whether current tree is exactly similar to the input tree that is
passed in or a tree similar to the input tree exists somewhere in left or right half of the currenttree. It makes use of `is_identical` function of the tree and recursion to compute the result.

### Below is the function defintion:

```rust, ignore
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
```
