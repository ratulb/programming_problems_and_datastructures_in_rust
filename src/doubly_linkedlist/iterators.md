# List iterators

We implement two variants of [iteratots](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for our list - one that evicts elements from the the list while the iterator is being traversed and the
other that does not mutate the underlying list. Both these variants can be traversed from the front
as well from the back because we implement [double ended iterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) for them.

First we will look at mutable variant - because if pretty simple.
