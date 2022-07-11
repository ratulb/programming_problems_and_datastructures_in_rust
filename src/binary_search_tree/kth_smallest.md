# Kth smallest

Here we present the iterative solution to find the kth smallest number. To do that we 
maintain a stack. We keep going left as long as possible and keep pushing the nodes we 
encounter (to back step) to the stack. When the lowest node is found - we pop from the 
stack and increment our internal counter. We check if interanl counter equals to the kth 
value we are looking for - If so, we return the node's key. Otherwise we start going up - 
either by going to the right or popping up previous node entry in the stack.

### Following snippet of code shows how we get the kth smallest iteratively:
```rust, ignore
    //kth smallest element - iterative
    pub fn kth_smallest(&self, k: usize) -> Option<T> {
        let mut curr = self.root();
        let mut stack = Vec::new();
        let mut n = 0;
        while curr.is_some() || !stack.is_empty() {
            while curr.is_some() {
                stack.push(curr.as_ref().cloned());
                curr = curr.and_then(|curr| curr.borrow().left_node());
            }
            curr = stack.pop().flatten();
            n += 1;
            if n == k {
                return curr.map(|curr| curr.borrow().key().clone());
            }
            curr = curr.and_then(|curr| curr.borrow().right_node());
	    }
        None
    }
```

