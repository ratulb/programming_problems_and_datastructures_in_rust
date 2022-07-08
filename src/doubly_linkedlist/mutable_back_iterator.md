# Mutable back iterator

Our `IntoIterator` implements `Iterator` - hence we can iterate forward traversing the elements.
What if we wanted to traverse the elements back? For that we have to implement [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html). To implement double ended
iterator it is mandatary that we implement `Iterator` first for our struct - whcih we have already
done.

To implement `DoubleEndedIterator` we need to provide an implementation of the `next_back` method
and our implementation is no-brainer - we just call pop_back on our list!

### Following is our implementation of DoubleEndedIterator:
```rust, ignore
impl<'a, T: std::fmt::Debug + Default + Clone + PartialEq> DoubleEndedIterator
    for IntoIterator<'a, T>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}
```


