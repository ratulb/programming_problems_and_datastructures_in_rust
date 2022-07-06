# Tree iterator

Calling `tree.iter()` returns an [iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) of `OptionM<T>` and `iterator.next()` returns the keys level wise. Also, `tree.iter()` can be 
called repeated since it does not consume the tree.

### Following are the relevant implementation details:

The `Iter` struct defintion:
```rust, ignore
#[derive(Debug)]
pub struct Iter<T: Ord + Default + Clone + std::fmt::Debug> {
    next: Option<VecDeque<Rc<RefCell<Node<T>>>>>,
}
```
The tree `iter` funtion:
```rust, ignore
    //Get an iterator for the tree's keys
    //Remember - calling iter on the tree would not consume the tree
    //iterator.next would return Option<T>
    //T is cloned
    //Keys would would be returned level wise
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.root().map(|node| {
                let mut next = VecDeque::new();
                next.push_front(node);
                next
            }),
        }
    }
```
The `Iterator` trait implementation:
```rust,ignore
impl<T: Ord + Default + Clone + std::fmt::Debug> Iterator for Iter<T> {
    type Item = T;
    //Level wise iterator
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(ref mut queue) => {
                let popped = queue.pop_back();
                match popped {
                    None => None,
                    Some(ref node) => {
                        let node = node.borrow();
                        if let Some(ref left) = node.left_node() {
                            queue.push_front(Rc::clone(left));
                        }
                        if let Some(ref right) = node.right_node() {
                            queue.push_front(Rc::clone(right));
                        }
                        Some(node.key.clone())
                    }
                }
            }
        }
    }
}
```
> **Note**: We cloning T when calling `iterator.next()`.
