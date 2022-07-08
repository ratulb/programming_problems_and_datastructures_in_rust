# Node and List

Below we present the defintion of `Node`. Member definitions are quite intuitive. We maintain a weak
previous reference to the preceeing `Node`.
```rust, ignore
#[derive(Debug, Default)]
struct Node<T: std::fmt::Debug + Default + Clone + PartialEq> {
    key: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}
//New up a new node with a value
impl<T: std::fmt::Debug + Default + Clone + PartialEq> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            next: None,
            prev: None,
        }
    }
}

//We implement `From` trait to wrap  up a node in RefCell and Rc
impl<T: std::fmt::Debug + Default + Clone + PartialEq> From<Node<T>>
    for Option<Rc<RefCell<Node<T>>>>
{
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}
```
Following is the defintion of `List`:
```rust, ignore
#[derive(Debug)]
pub struct List<T: std::fmt::Debug + Default + Clone + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}
```





