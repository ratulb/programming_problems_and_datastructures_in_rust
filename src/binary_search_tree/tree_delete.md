# Tree delete

While deleting a node from the tree - we first find the target node that is getting deleted. If it
can not be found - we have no further business -  we return `None`.

If we find a target node to delete - we need to consinder few scenarios:
* Is it the root node that we are deleting? Then target will have no parent.
  * It has no child - take node out - leaving the tree empty.
  * It has left child - hoist left child up - take out its parent reference.
  * It has right child - hoist right child up - take out its parent reference.
  * It has both children - delegate to `Node::delete`
* It is not the root node. Then the target has got a parent.
  * It has no child or has left or right child - delegte to `node.delete_child`
  * It has both children - delegate to `Node::delete`
 
### Below is the tree delete function in entirety:

```rust, ignore
//Delete a node with key that equals the supplied key
    //Returns the deleted key as Some(key) or None otherwise
    pub fn delete(&mut self, key: &T) -> Option<T> {
        let target = Self::find(self, key);
        match target {
            None => None,
            Some(ref node) => {
                let has_left = node.borrow().has_left();
                let has_right = node.borrow().has_right();

                let has_both = has_left && has_right;
                let no_child = !has_left && !has_right;
                let has_parent = node.borrow().parent.is_some();
                match has_parent {
                    false => {
                        //Delete root - root has no parent ref - hence differential treatment
                        match (no_child, has_left, has_right, has_both) {
                            (true, false, false, false) => {
                                self.0.take().map(|root| root.take().key)
                            }
                            //Has left child - remove left child's parent ref and set it as
                            //tree root
                            (false, true, false, false) => {
                                let root = self.root().take();
                                self.0 = root.as_ref().and_then(|root| {
                                    root.borrow().left_node().map(|node| {
                                        node.borrow_mut().parent.take();
                                        node
                                    })
                                });
                                //Return root's key
                                root.map(|root| root.take().key)
                            }
                            //Has right child - remove right child's parent ref and set it as
                            //tree root
                            (false, false, true, false) => {
                                let root = self.root().take();
                                self.0 = root.as_ref().and_then(|root| {
                                    root.borrow().right_node().map(|node| {
                                        node.borrow_mut().parent.take();
                                        node
                                    })
                                });
                                //Return root's key
                                root.map(|root| root.take().key)
                            }
                            //Has got both children - delete to Node::delete
                            (false, true, true, true) => Node::delete(target),
                            (_, _, _, _) => None,
                        }
                    }
                    //target node being deleted has got a parent
                    true => match (no_child, has_left, has_right, has_both) {
                        (true, false, false, false)
                        | (false, true, false, false)
                        | (false, false, true, false) => {
                                let root = self.root().take();
                                self.0 = root.as_ref().and_then(|root| {
                                    root.borrow().right_node().map(|node| {
                                        node.borrow_mut().parent.take();
                                        node
                                    })
                                });
                                //Return root's key
                                root.map(|root| root.take().key)
                            }
                            //Has got both children - delete to Node::delete
                            (false, true, true, true) => Node::delete(target),
                            (_, _, _, _) => None,
                        }
                    }
                    //target node being deleted has got a parent
                    true => match (no_child, has_left, has_right, has_both) {
                        (true, false, false, false)
                        | (false, true, false, false)
                        | (false, false, true, false) => {
                            let parent = node.borrow().upgrade_parent();
                            //Is it left or right? If no child then left is false
                            let left = parent
                                .as_ref()
                                .map_or(false, |parent| parent.borrow().is_left_child(key));
                            //Delefate to node.delete_child with boolean flag left
                            parent.and_then(|parent| parent.borrow_mut().delete_child(left))
                        }
                        //Has parent, has two children
                        //Delegate to Node::delete
                        (false, true, true, true) => Node::delete(target),
                        (_, _, _, _) => None,
                    },
                }
            }
        }
    }
```
Next we look at some other API's that the tree struct exposes.


