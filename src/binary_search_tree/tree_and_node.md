# Tree and Node definition

### We define our Tree as follows:

```rust ,ignore
#[derive(Debug, Default)]
pub struct Tree<T: Ord + Default + Clone + std::fmt::Debug>(Option<Rc<RefCell<Node<T>>>>);

```
Our Tree is tuple struct which may or may not contain a Node. The Node itself is wrapped inside a
RefCell for interior mutability. The RefCell, in turn is wrapped inside a Rc for shared access.

Our tree's keys (aka entries) are generic T. T must be of type `Ord` because that is how we decide
which side of the tree an entry lands in.

The generic type `T` is also implements `Default`. This we are utilising for flushing out key values while deleting a Node from the tree. This becomes necessary because rust is grumpy about holes in
memory.

Clone was not strictly necessary - but we need it later - when we implement `Iterator` for the
tree.

Debug is, of course, for printing the tree. Next, we define our `Node`.


### We define our Node as follows:
```rust ,ignore
struct Node<T: Ord + Default + Clone + std::fmt::Debug> {
    key: T,
    left: Option<Rc<RefCell<Tree<T>>>>,
    right: Option<Rc<RefCell<Tree<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

```
Node defintion is easy to understand. We have `key` of type T. Keys are what we insert into the tree.
We have left and right children - which are optional Trees contained with `RefCell` and `Rc`. We are
treating each child as a `Tree` in its own right.

Our parent is an optional `Weak` reference contained within a `RefCell`.

> **Note**: All nodes will have a weak referene to a parent node except for root which would have no
parent

Next we will look at some basic API's for `Tree` and `Node`.


