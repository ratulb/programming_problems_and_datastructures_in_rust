# A rust binary search with parent pointers

## Why binary search tree?

Binary search trees are useful - very useful. They offer fast insertion and deletion time and
unlike linked list provide fast lookup time. Arrangement of entries are ordered. By convention -
for a given entry, smaller entries are always on its left and biggers are always on the right.

## Binary search tree implementation in rust

Binary search trees (henceforth called BSTs) can be implemented in rust without parent pointers.
But deletion of an entry becomes somewhat clunky without parent pointers.

Here we implement a BST in rust with parent pointers. Both left and right child point to their 
parent. We make use of [shared references](https://doc.rust-lang.org/std/rc/struct.Rc.html) -
since same parent is shared by both children as well as we need to access it from multiple contexts.
Reference counted pointer alone - [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) - is not very
useful in itself - atleast in this context - since we 'Rc' provides mutable reference to its inner
content - only if there is no other existing reference to it already. This, obviously, is not 
the case here - because children will be pointing to it all the time. We need to be able to
mutate (for example -change its value or delete) it - while there are outstanding child references
to a it. We need [shared interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html). This is where [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) comes handy - it provides interior mutability when no existing 'borrow' (mutable and immutable) is in the
context and multiple shared references when there exists no mutable borrow.

One last thing though. If parent contains(optionally) a child and child points back at its parent,
 while deleting a parent - its reference count will still be 1(or 2). Since parent's reference count is not 0 - it would not be dropped - it would be lingering in memory leading to a [leaky situation](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html). This is certainly undesirable and we
don't want this. Which is why we want our parent pointers to be [weak](https://doc.rust-lang.org/std/rc/struct.Weak.html) ones so that we can drop parents even when there are outstanding weak references to it.

With these understanding so far - next we define our Tree and Node. 


