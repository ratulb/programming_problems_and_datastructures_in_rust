# Immutable bi-directional iterator

We want to be able to iterate the elemnts of our list as many times as we want - but our list
should not change. We take an iterator from out list, travese the elements, exhaust the iterator,
take another one and so on. This where immutable iterator comes in. Immutable iterator generally
provides an access to `Option<&T>`. We can not alter underlying `List` with an immutable shared
`Option<&T>`. We can read or may be clone it - if clone is supported for `T`.

In our case, we are breaking rust convention again - instead of returning `Option<&T>`, we are
going to return `Option<T>`. But that does not change anything about the immutability of our
underlying `List` - it would still remain intact. We clone our `&T` and return `Option<T>`.

The reason for returning `Option<T>` and not `Option<&T>` is that our `T`s are burried deep inside
of `RefCell`s which themselves are insides' of `Rc`s. Its seems pretty damn hard to give a `&T` out
from `Rc<RefCell<Node<T>>>` - I have not figured it out yet - not sure if that would be possible 
without resorting to unsafe rust.

Hence we stick it our good friend - `Option<T>`.

In the previous section - we have implemented `DoubleEndedIterator` when we had a mutable referece
to our underlying `List`. It was easy because we made use of `pop_front` and `pop_back` and we were
able get elements from right ends in right order.

In the immutable case, we don't want to maintain a mutable reference to our underlying list. We
aslo want our immutable iterator to be able traverse forward as well as backward and while doing so, we don't want to trample each other. In another words, while calling `next`, we don't want our
iterator to return elements which we have already seen by calling `next_back` and vice versa.

Again, when we call `iter` on our list to get a bi-directional iterator - the struct that gets 
returned should be simple enough and should not have lot of code to achieve the funtionality that 
we want. We want to handle case whether to return an element or not, because it may already have 
been returned by `next_back` or `next`, internally. We want that piece of logic to reside in some 
other struct. Ok, enough talk. Let's get down to code.

### Immutable iterator struct defintion that caller gets:
```rust, ignore
pub struct Iter<T: std::fmt::Debug + Default + Clone + PartialEq> {
    nodes: NodeIterator<T>,
}
```
Above struct holds an instance of `NodeIterator` that has head and tail of the list as members.
### Iterator trait implementation for `Iter`:
```rust, ignore
//Itearor that returns Option<T>
//Values are cloned
//Underlying list remain intact
impl<T: std::fmt::Debug + Default + Clone + PartialEq> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes
            .next()
            .as_ref()
            .map(|next| next.borrow().key.clone())
    }
}
```
### DoubleEndedIterator implementation:
```rust, ignore
//Iterate back 
//Calling next_back should not returned elements seen by calling next and vice versa
impl<T: std::fmt::Debug + Default + Clone + PartialEq> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nodes
            .next_back()
            .as_ref()
            .map(|prev| prev.borrow().key.clone())
    }
}
```
Our iterator and double ended iterator implementations are simple enough and much of the logic is 
hidden inside `NodeIterator`. But it turns out our `NodeIterator` is also not very complex at all.

### NodeIterator struct defintion:
```rust, ignore
//This struct holds head and tail of the list
//ptr_crossed flag indicates whether front and back iterators crossed each other
struct NodeIterator<T: std::fmt::Debug + Default + Clone + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    ptr_crossed: bool,
}
```
### Iterator trait implementation for NodeIterator:

`NodeIterator` struct has references to head and tail of the list. These references move forward 
and backward as we call `next` and `next_back`. When calling `next` - we see if the `ptr_crossed` 
flag is set - if set we return `None`, if not set - we proceed forward - check if we are returing 
the same element that tail is pointing at - if so, if set the `ptr_crossed` flagged is set so that  calling `next_back` does not return any more elements. That's all to it. `DoubleEndedIteraror` 
implementation does same thing just that it looks at the head pointer.
```rust, ignore
impl<T: std::fmt::Debug + Default + Clone + PartialEq> Iterator for NodeIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(_) => {
                match self
                    .head
                    .as_ref()
                    .map(|next| (Rc::clone(next), next.borrow().next.as_ref().map(Rc::clone)))
                {
                    None => None,
                    Some(this_and_next) => match self.ptr_crossed {
                        true => None,
                        false => {
                            let this = this_and_next.0;
                            let next = this_and_next.1;
                            self.ptr_crossed = self
                                .tail
                                .as_ref()
                                .map(|tail| Rc::ptr_eq(&this, tail))
                                .unwrap_or(false);
                            self.head = next;
                            Some(this)
                        }
                    },
                }
            }
            None => None,
        }
    }
}
```
### DoubleEndedIterator implementation for NodeIterator:
```rust, ignore
impl<T: std::fmt::Debug + Default + Clone + PartialEq> DoubleEndedIterator for NodeIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.tail {
            Some(_) => {
                match self.tail.as_ref().map(|tail| {
                    (
                        Rc::clone(tail),
                        tail.borrow()
                            .prev
                            .as_ref()
                            .cloned()
                            .and_then(|prev| prev.upgrade()),
                    )
                }) {
                    None => None,
                    Some(this_and_prev) => match self.ptr_crossed {
                        true => None,
                        false => {
                            let this = this_and_prev.0;
                            let prev = this_and_prev.1;
                            self.ptr_crossed = self
                                .head
                                .as_ref()
                                .map(|head| Rc::ptr_eq(&this, head))
                                .unwrap_or(false);
                            self.tail = prev;
                            Some(this)
                        }
                    },
                }
            }
            None => None,
        }
    }
} 
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/doubley_linked_list/src/lib.rs)
