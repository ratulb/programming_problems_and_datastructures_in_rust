# Tree validation

Below we present the implementation of an iterative algoritm to validate the binary search tree. The idea here is to go the left most node - and then traverse the tree in order. 
When traversing tree in order - we expect every next element to be bigger than the previous element and lesser than the root element. When we cross the root - we want every element 
be greater than the root and its previous element.

###Following function achieves the same:
```rust, ignore
//Validate the if the tree is valid
    //We use an interative process here
    pub fn validate(&self) -> bool {
        match self.0 {
            None => true,
            Some(ref root) => {
                let mut current = Some(Rc::clone(root));
                let mut stack = Vec::new();
                let mut previous = None;
                let mut crossed_root = false;
                let mut first = Some(false);
                while current.is_some() || !stack.is_empty() {
                    while current.is_some() {
                        stack.push(current.as_ref().cloned());
                        current = current.and_then(|inner| inner.borrow().left_node());
                    }
                    current = stack.pop().and_then(|popped| popped);
                    current = stack.pop().and_then(|popped| popped);
                    //Has moved to the right side of the tree
                    if !crossed_root {
                        crossed_root = current.as_ref().map(|curr| Rc::ptr_eq(curr, root)).unwrap();
                        if let Some(first) = first.as_mut() {
                            *first = crossed_root;
                        }
                    }
                    match previous {
                        None => previous = current.as_ref().map(Rc::clone),
                        Some(ref prev) => match current.as_ref().map(|curr| {
                            (
                                crossed_root,
                                &mut first,
                                curr.borrow().key() > prev.borrow().key(),
                                curr.borrow().key() < root.borrow().key(),
                            )
                        }) {
                            Some((false, Some(false), true, true)) => {
                                previous = current.as_ref().map(Rc::clone)
                            }
                            Some((true, Some(true), true, false)) => {
                                if let Some(first) = first.as_mut() {
                                    *first = false;
                                }
                                previous = current.as_ref().map(Rc::clone)
                            }
                            Some((true, Some(false), true, false)) => {
                                previous = current.as_ref().map(Rc::clone)
                            }
                            _ => return false,
                        },
                    }
                    current = current.and_then(|inner| inner.borrow().right_node());
                }
                true
            }
        }
    }
```
