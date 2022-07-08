# Delete helpers

To figure out whether a target node that is being deleted is the first or last node in the list,
we compare it to the head or the tail of the list. This comparsion we do by using the [ptr_eq](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq).

### Below are the codes for achieving the same:
```rust, ignore
    //Is the passed in node reference the first in the list?
    fn is_first(&self, node: &Rc<RefCell<Node<T>>>) -> bool {
        match self.head {
            None => false,
            Some(ref head) => Rc::ptr_eq(head, node),
        }
    }

    //Is the passed in node reference the last in the list?
    fn is_last(&self, node: &Rc<RefCell<Node<T>>>) -> bool {
        match self.tail {
            None => false,
            Some(ref tail) => Rc::ptr_eq(tail, node),
        }
    }
```
One we have figured that a target node that is being deleted is either first or last, its just a
matter of calling `pop_front` or `pop_back` accordingly.

### Deleting a inner target node:

To delete inner target node we first get a handles to its previous and next nodes. Once we find them,
we need to rewire the linkages appropriately.

We clone the the outgoing target's previous pointer, which is a weak pointer reference, set it as theprevious pointer of the outgoing node's next node.

We upgrade previous (`prev`) weak  pointer to get a strong refernce to get to previous node and set 
next's cloned reference as its next. Then we return the content of outgoing target node.

Following is snippet for deleting an inner node:
```rust, ignore
//Delete a node that has previous and next
    fn delete_inner(&mut self, target: &Rc<RefCell<Node<T>>>) -> Option<T> {
        let prev = target.borrow_mut().prev.take();
        let next = target.borrow_mut().next.take();
        if let Some(ref next) = next {
            next.borrow_mut().prev = prev.as_ref().cloned();
        }
        if let Some(ref prev) = prev {
            if let Some(prev) = prev.upgrade() {
                prev.borrow_mut().next = next.as_ref().map(Rc::clone);
            }
        }
        Some(target.take().key)
    }
```
Now, that we know how the helpers work, we present the actual `delete` function - which is quite
simple.

### Delete public API:
```rust, ignore
    //Delete a key from the list. We try to find the by using iterator `find` method.
    //If found - we check if it is the first or last key in the list. If the found node
    //happens to be first or last - we call pop_front or pop_back as required.
    //If the key is in between head and and tail - the deletion is handled accordingly.
    pub fn delete(&mut self, key: &T) -> Option<T> {
        match self.node_iter().find(|node| node.borrow().key == *key) {
            None => None,
            Some(ref target) => match (self.is_first(target), self.is_last(target)) {
                (true, true) | (true, false) => self.pop_front(),
                (false, true) => self.pop_back(),
                (_, _) => self.delete_inner(target),
            },
        }
    }
```
> **Note**: To find the target node that is being deleted, we are making use of `Iterator`'s [find](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) api. We have not talked about
`List`'s iterator implementation yet - which we will turn to next.
`

