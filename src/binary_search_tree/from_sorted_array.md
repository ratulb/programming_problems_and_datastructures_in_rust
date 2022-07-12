# From sorted array

Here we present how to create a height balanced binary search tree from a sorted array. 
Height balanced BST is such that - height of subtrees in the left and right side of the 
root would not differ by more than 1. The algorithm we use is recursive.

We pick the mid point of the sorted array - make it the root of the tree. Then we - take 
half from next of the mid point till the end the array - make a recursive call to make the right subtree of them root. We do the same thing for left subtree - only difference is 
that this time we take the half from start of the array till the element before the mid 
point.

### Following is the implementation:
```rust, ignore
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
```
> **Note**: We are making use of nested functions above.
