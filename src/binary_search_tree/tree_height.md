# Tree height

### Tree `height` function:
```rust, ignore
 //Find the height of the tree
    pub fn height(&self) -> usize {
        let root = self.root();
        match root {
            None => 0,
            Some(ref node)
                if node.borrow().left_node().is_none() & node.borrow().right_node().is_none() =>
            {
                1
            }
            Some(ref node) => {
                let left_tree_height = node
                    .borrow()
                    .left
                    .as_ref()
                    .map(|tree| Self::height(&tree.borrow()))
                    .unwrap_or(0);
                let right_tree_height = node
                    .borrow()
                    .right
                    .as_ref()
                    .map(|tree| Self::height(&tree.borrow()))
                    .unwrap_or(0);
                1 + std::cmp::max(left_tree_height, right_tree_height)
            }
        }
    }
```

