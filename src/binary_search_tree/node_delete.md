# Node delete (Node has two children)

`Node::delete` function is invoked when target node being deleted has both right and left chidren.
We find the minimum node on the right of target. Once we find the minimum, then we find its parent
and convert parent weak reference to a strong reference.

Next we find the right child of min if any - this right child should be now hoisted to point to min'sparent since min is being evicted to replace deleted node's content with min's content.

Finally. we return the target node's content from this function.

### Following is the whole `Node::delete` function with inline comments:

```rust, ignore
//Delete an node when it has two children
    fn delete(mut target: Option<Rc<RefCell<Node<T>>>>) -> Option<T> {
        let min = target
            .as_ref()
            .and_then(|target| target.borrow().right_node().as_ref().and_then(Tree::min));
        let min_parent = min.as_ref().and_then(|min| min.borrow().upgrade_parent());
        let right_parent = Node::right_parent(target.as_ref(), min_parent.as_ref());
        let left_or_right = right_parent.as_ref().and_then(|parent| {
            min.as_ref()
                .map(|min| parent.borrow().is_left_child(min.borrow().key()))
        });
        let min = match right_parent {
            None => None,
            Some(parent) => match left_or_right {
                None => None,
                Some(true) => parent.borrow_mut().left.take(),
                Some(false) => parent.borrow_mut().right.take(),
            },
        };
        let mut min = min.map(|tree| tree.take()).and_then(|tree| tree.root());
        let min_right = min
            .as_ref()
            .and_then(|min| min.borrow_mut().right.take())
            .and_then(|tree| tree.borrow().root());
        if let Some(ref min_right) = min_right {
            min_right.borrow_mut().parent = right_parent
                .as_ref()
                .map(|parent| Rc::downgrade(&Rc::clone(parent)));
        }
        if let Some(parent) = right_parent {
            match left_or_right {
                None => {}
                Some(true) => {
                    parent.borrow_mut().left = Tree::with_node(min_right.as_ref().cloned())
                }
                Some(false) => {
                    parent.borrow_mut().right = Tree::with_node(min_right.as_ref().cloned())
                }
            }
        }
        match target {
            Some(ref mut target) => target
                .borrow_mut()
                .replace_key(min.take().map(|min| min.take().key)),
            None => None,
        }
    }
}
```

The above function takes care of deleting a target node when it has both children. We have separated
out the code to handle the case when the deleted node has only one or no child but has got a parent. Let's take a look at that in the next section.
