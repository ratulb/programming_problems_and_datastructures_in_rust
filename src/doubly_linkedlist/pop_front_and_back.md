# Pop front and pop back

While popping from the front of the list, we take out existing head, make it's next the new head of
the list. While doing so, we also take out previous pointer of the new head, which was pointing at
the outgoing head. If the new head turns out to be `None` - we also set list tail to `None`. We
return the outgoing head's content.

### Below is the code for popping from the front:
```rust, ignore
    //Pop out from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(ref mut head) => {
                self.head = head.borrow_mut().next.take().map(|next| {
                    next.borrow_mut().prev.take();
                    next
                });
                if self.head.is_none() {
                    self.tail.take();
                }
                //Use of default
                Some(head.take().key)
            }
        }
    }
```
While popping from the back, we take out the list's tail, convert the tail's previous pointer to a
strong reference, make the previous the new tail of the list. If the new tail turns out to be `None`
we set head of the list to `None` as well. Finally, we return the outgoing tail's content from the
call.

### Belwo is the code for popping from the back:
```rust, ignore
   //Pop out from the back of the list
    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            None => None,
            Some(ref mut tail) => {
                self.tail = tail.borrow().prev.as_ref().and_then(|prev| {
                    prev.upgrade().map(|prev| {
                        prev.borrow_mut().next = None;
                        prev
                    })
                });
                if self.tail.is_none() {
                    self.head.take();
                }
                //Use of default
                Some(tail.take().key)
            }
        }
    }
```
