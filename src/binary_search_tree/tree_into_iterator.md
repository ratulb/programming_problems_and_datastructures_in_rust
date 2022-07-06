# Tree into iterator

Unlike  `tree.iter()` `tree.into_iter()` returns an iterator of `Option<T>` that consumes the tree
elements one by one when called `next` on it. Also, every time `next` is called, the root of the
tree is popped.

### Following sre the revelvant definitions:

`IntoIter` struct defintion:
```rust, ignore
#[derive(Debug)]
pub struct IntoIter<'a, T: Ord + Default + Clone + std::fmt::Debug> {
    tree: Option<&'a mut Tree<T>>,
}
```
Tree `into_iter` funtion:
```rust,ignore
//Returns an iterator that consumes the tree elements one by one
//when calling next on it
//Root of the tree is eviced when next is called on the iterator
pub fn into_iter(&mut self) -> IntoIter<'_, T> {
        IntoIter {
            tree: match self {
                Tree(None) => None,
                Tree(_) => Some(self),
            },
        }
    }
```
Iterator trait implementation for `IntoIter`:
```rust, ignore
impl<T: Ord + Default + Clone + std::fmt::Debug> Iterator for IntoIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.tree {
            None => None,
            Some(ref mut tree) => match tree.0 {
                None => None,
                Some(ref mut node) => {
                    let key = node.borrow().key.clone();
                    tree.delete(&key);
                    Some(key)
                }
            },
        }
    }
}
```


