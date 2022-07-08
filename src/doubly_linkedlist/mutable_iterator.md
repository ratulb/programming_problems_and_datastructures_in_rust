# Mutable iterator

We iterate through iterator elements by calling [next](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next) on it - this means we need to maintain the iterator's state. Folloing is a
struct that mutabley borrows the underlying list and stores it. The life time paramter 'a is there 
because we are holding a reference to the list - we don't own the list. An `IntoIterator` instance 
can exist only as long as unerlying list struct instance is alive. The iterator can not outlive the
list struct instance that it is borrowing.
```rust, ignore
pub struct IntoIterator<'a, T: std::fmt::Debug + Default + Clone + PartialEq> {
    list: &'a mut List<T>,
}
```
### Getting an IntoIterator instance:

We have added following function to the `List` struct to return an `IntoIterator` instance. While
returning the `IntoIterator` struct instance - list passes it its own mutable self reference. And
because we have a mutable reference to the list - we can modify it as long as we return it back in
some shape - either intact or altered.
```rust, ignore
pub fn into_iter(&mut self) -> IntoIterator<'_, T> {
   //Taking a mutable reference to the list
   IntoIterator { list: self }
}
```
> **Node**: We are breaking rust's convention here. Idiomatic rust consumes the collection as a
whole if elements are dropped while iterating. Here we are not destroying the list though post
iteration it bcomes empty. We could have consumed the list instead.  

Now that we have struct that we have named `IntoIterator` and it has a mutable list inside it - can
we iterate over the elements inside the list? Not yet. We have not added the iterator behaviour to
our struct yet. We need to implement the [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait for our `IntoIterator` struct for that. Whichis what we will do next.

> **Note**: The `Iterator` trait has quite a bunch of methods defined in it. But we need to 
implement only the `next` method of it because rests are implemented based on `next`.

### Implement Iterator for IntoIterator:
```rust, ignore
//Iterator that consumes the list elements from the front
impl<'a, T: std::fmt::Debug + Default + Clone + PartialEq> Iterator for IntoIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}
```
Our implementation could not have been any more simpler than this - we just pop the elements from
the front! 
