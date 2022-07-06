# Node delete (Node has two children)

`Node::delete` function is invoked when target node being deleted has both right and left chidren.
We find the minimum node on the right of target. Once we find the minimum, then we find its parent
and convert parent weak reference to a strong reference.

Next we find the right child of min if any - this right child should be now hoisted to point to min'sparent since min is being evicted to replace deleted node's content with min's content.

Finally. we return the target node's content from this function.

### Following is the whole `Node::delete` function with inline comments:

```rust, ignore

    //Delete a target node - gets invoked when the target node has both left
    //and right node
    fn delete(mut target: Option<Rc<RefCell<Node<T>>>>) -> Option<T> {
        //Find the min node in the right side of the target node that is being
        //deleted
        let min = target
            .as_ref()
            .and_then(|target| target.borrow().right_node().as_ref().and_then(Tree::min));
        //Find strong reference(upgradded from weak) to min's parent
        let min_parent = min.as_ref().and_then(|min| min.borrow().upgrade_parent());
        //Find the right child of min if any. Once min is taken out to fill the 
        //deleted target node's content with evicted min node's content, min's right should
        //be pointing at min's parent
        let mut min_right_child = min.as_ref().and_then(|min| {
            min.borrow_mut()
                .right
                .take()
                .as_ref()
                .and_then(|child| child.borrow().root())
        });
        //Make min's right point to min's parent
        if let Some(ref mut child_node) = min_right_child {
            child_node.borrow_mut().parent = min
                .as_ref()
                .and_then(|min| min.borrow().parent().as_ref().cloned());
        }
        //min's parent could be the target node being deleted or some other node on the far
        //right of it. Choose the appropriate parent
        let mut right_parent = Node::right_parent(target.as_ref(), min_parent.as_ref());
        //Set min's right as the right tree of min's parent
        if let Some(ref mut parent) = right_parent {
            parent.borrow_mut().right =
                min_right_child.map(|right_child| Rc::new(RefCell::new(Tree(Some(right_child)))));
        }
        //Return the key of the target node being deleted
        match target {
            Some(ref mut target) => target
                .borrow_mut()
                .replace_key(min.map(|min| min.take().key)),
            None => None,
        }
    }
```

The above function takes care of deleting a target node when it has both children. We have separated
out the code to handle the case when the deleted node has only one or no child but has got a parent. Let's take a look at that in the next section.
