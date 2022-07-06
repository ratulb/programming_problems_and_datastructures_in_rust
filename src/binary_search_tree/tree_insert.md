# Tree population

The `insert` function in the `Tree` struct is responsible for populating the tree. It's straight-
forward - it looks at the current tree's root - if it does not exist - creates a new `Node` with
the given input key - sets it as the root node of the tree.

If the current tree's root exists, the input key is compared to the root's key and the decision
is made to as to whether to go the left or right of the tree. This is done recursively. Once the
right spot is found - a new `Tree` branch is created with the input key, its parent is set and the
newly created tree is added as left or right child of the parent.

> **Note**: We downgrade parent's reference because children always maintain a weak pointer to its
parent. Also, we don't allow duplicate entry. 

### Following is `insert` function in its entirety:

```rust, ignore
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
```
> **Note**: We are using `Node::new(value)`, `Tree::new_branch(right)` and `Node::wrapped_node(value)` from the previous Basic APIs section here.

Next we present the `Tree::find` function which is utilized to find a target `Node` that has to be
deleted.
