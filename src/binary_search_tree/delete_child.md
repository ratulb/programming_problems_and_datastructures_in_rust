# Node delete (One or no child)

Node `delete_child` function gets invoked when the target node being deleted has only left or right
child or no child but has got a parent. We evict the target node and take out its key, parent and
its left or right child in a tuple.

Next we set the deleted node's left or right child's parent to deleted node's parent and finally 
return the deleted node's key.

### Following is the snippet for `delete_child` function:

```rust, ignore

    //Delete a node with single child or no child but node being deleted has parent
    //left: bool -> Should we delete left or right child?
    fn delete_child(&mut self, left: bool) -> Option<T> {
        //First take out the left or right child based on the flag passed in
        let deleted = match left {
            true => self.left.take(),
            false => self.right.take(),
        };
        //result is tuple of the form as shown below
        //result = (deleted.key, deleted.parent, left or right child of deleted)
        let result = match deleted
            .and_then(|tree| tree.take().0)
            .map(|node| node.take())
            .map(|node| (node.key, node.parent, node.left.or(node.right)))
        {
            //Set deleted node's left or right child's parent to the parent of deleted
            Some((key, parent, mut tree)) => {
                if let Some(ref mut inner) = tree {
                    if let Some(ref mut tree_node) = inner.borrow_mut().0 {
                        tree_node.borrow_mut().parent = parent;
                    }
                }
                //(deleted.key, left or right of deleted
                (Some(key), tree)
            }
            None => (None, None),
        };
        //Set self left right to deleted left or right
        match left {
            true => self.left = result.1,
            false => self.right = result.1,
        }
        //deleted key
        result.0
    }

```

Now that we have discussed `Node`'s delete APIs - we can go back to `Tree`'s delete API. Lets do thatnext.

