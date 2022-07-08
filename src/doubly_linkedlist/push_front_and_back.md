# Push front and push back

### Push to the front of the list:
```rust, ignore
//Push to the front of the list
    pub fn push_front(&mut self, key: T) {
        let node = Node::new(key).into();
        match self.head {
            None => {
                self.head = node;
                self.tail = self.head.as_ref().map(Rc::clone);
            }
            Some(ref mut head) => {
                head.borrow_mut().prev = node.as_ref().map(|node| Rc::downgrade(&Rc::clone(node)));
                self.head = node.map(|node| {
                    node.borrow_mut().next = Some(Rc::clone(head));
                    node
                });
            }
        }
    }
```
Above, we create a new node, set it as head and tail - if the list is empty. Otherwise, make the new node the head of the list, make the existing head to point to it via a weak reference and make new node's `next` point to the existing head.

### Push to the back of the list:

Again, we set the newly created node as the `head` and `tail` of the list - if the list is empty.
Otherwise, make the new node point to existing tail via a weak reference, make the existing tail to
point to new node and set new node as the tail of the list.

### Code for pushing to the back of the list:
```rust, ignore
//Push to the back of the list
    pub fn push_back(&mut self, key: T) {
        let mut node = Node::new(key).into();
        match self.tail {
            None => {
                self.head = node;
                self.tail = self.head.as_ref().map(Rc::clone);
            }
            Some(ref mut tail) => {
                self.tail = node.take().map(|node| {
                    node.borrow_mut().prev = Some(Rc::downgrade(&Rc::clone(tail)));
                    tail.borrow_mut().next = Some(Rc::clone(&node));
                    node
                });
            }
        }
    }
```
Next, we would look at `pop_front` and `pop_back`.

