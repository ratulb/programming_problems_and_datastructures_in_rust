# Nth smallest

To find nth smallest key in the tree we go all the way to the left. Once we are there, we 
ask the question - is the current position is the nth position? If so, we return the node's key at that position. Otherwise, we move up - repeat the process.

### Following is the routine for finding the nth smallest:
```rust, ignore
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
