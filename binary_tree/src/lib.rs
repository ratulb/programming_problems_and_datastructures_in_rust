use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Node<K: Ord> {
    pub val: K,
    pub left: Option<Box<Node<K>>>,
    pub right: Option<Box<Node<K>>>,
}
impl<K: Ord + Default + Clone + std::fmt::Debug + std::fmt::Display> Node<K> {
    #[inline]
    pub fn new(val: K) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: K) {
        if value < self.val {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    pub fn level_order(&self) {
        let mut queue = VecDeque::new();
        queue.push_front(self);
        while !queue.is_empty() {
            let node = queue.pop_front();
            if let Some(ref node) = node {
                if let Some(ref left) = node.left {
                    queue.push_back(left);
                }
                if let Some(ref right) = node.right {
                    queue.push_back(right);
                }
                println!("{:?}", node.val);
            }
        }
    }

    pub fn exists(&self, key: K) -> bool {
        if self.val == key {
            return true;
        } else if self.val < key {
            match self.right {
                Some(ref right) => right.exists(key),
                None => false,
            }
        } else {
            match self.left {
                Some(ref left) => left.exists(key),
                None => false,
            }
        }
    }

    pub fn find(&self, key: K) -> Option<&Self> {
        if key == self.val {
            return Some(self);
        } else if key > self.val {
            return match self.right {
                Some(ref right) => right.find(key),
                None => None,
            };
        } else {
            return match self.left {
                Some(ref left) => left.find(key),
                None => None,
            };
        }
    }

    pub fn height(&self) -> usize {
        if self.left == None && self.right == None {
            return 1;
        }

        let l = match self.left {
            Some(ref l) => Self::height(l),
            None => 0,
        };
        let r = match self.right {
            Some(ref r) => Self::height(r),
            None => 0,
        };
        return std::cmp::max(l, r) + 1;
    }

    pub fn delete(&mut self, key: &K) -> Option<Self> {
        if self.val > *key {
            match self.left {
                Some(ref mut l) => {
                    self.left = match Self::delete(l, key) {
                        Some(pd) => Some(Box::new(pd)),
                        None => return None,
                    }
                }
                None => return None,
            }
        } else if self.val < *key {
            match self.right {
                Some(ref mut r) => {
                    self.right = match Self::delete(r, key) {
                        Some(pd) => Some(Box::new(pd)),
                        None => return None,
                    }
                }
                None => return None,
            }
        } else {
            if self.left == None && self.right == None {
                std::mem::drop(self);
                return None;
            } else if self.left == None {
                *self = *self.right.take().unwrap();
            } else if self.right == None {
                *self = *self.left.take().unwrap();
            } else {
                if let Some(ref mut r) = self.right {
                    let min = Self::find_min_node(r);
                    println!("I got called");
                    self.val = min.val.clone();
                    self.right = Self::delete(r, &min.val).map(|n| Box::new(n));
                }
            }
            return Some(std::mem::take(self));
        }
        None
    }

    pub fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        print!("{:?} ", self.val);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }

    pub fn reverse_in_order(&self) {
        if let Some(ref right) = self.right {
            right.reverse_in_order();
        }
        print!("{:?} ", self.val);
        if let Some(ref left) = self.left {
            left.reverse_in_order();
        }
    }

    pub fn min(&self) -> &K {
        let mut current = self;
        let mut min = &current.val;
        while let Some(ref left) = current.left {
            min = &left.val;
            current = match left.left {
                Some(ref next_left) => {
                    min = &next_left.val;
                    next_left
                }
                None => return min,
            };
        }
        min
    }

    pub fn find_min_node(&self) -> Self {
        let mut current = self;
        let mut min = current;
        while let Some(ref left) = current.left {
            min = left;
            current = match left.left {
                Some(ref next_left) => {
                    min = next_left;
                    next_left
                }
                None => return min.clone(),
            };
        }
        min.clone()
    }

    pub fn min_node(node: Node<K>) -> Self {
        let mut current = node;
        if let Some(left) = current.left {
            current = Self::min_node(*left);
        }
        current
    }

    pub fn min_node_mut(node: &mut Node<K>) -> Self {
        let mut current = node;
        if let Some(ref mut left) = current.left {
            *current = Self::min_node_mut(left);
        }
        std::mem::take(current)
    }

    pub fn min_node_mut_self(&mut self) -> Self {
        let mut current = self;
        if let Some(ref mut left) = current.left {
            *current = Self::min_node_mut_self(left);
        }
        std::mem::take(current)
    }

    pub fn min_node_match(&mut self) -> Self {
        let mut current = self;
        while let Some(ref mut left) = current.left {
           *current = Self::min_node_match(left);
        }
        std::mem::take(current)
    }

    pub fn max(&self) -> &K {
        let mut current = self;
        let mut max = &current.val;
        while let Some(ref right) = current.right {
            max = &right.val;
            current = match right.right {
                Some(ref next_right) => {
                    max = &next_right.val;
                    next_right
                }
                None => return max,
            };
        }
        max
    }

    pub fn all_paths(&self, path: &mut String, paths: &mut Vec<String>) {
        let curr = self.val.to_string();
        path.push_str(&curr);
        if self.left.is_none() && self.right.is_none() {
            paths.push(path.to_string());
            return;
        }
        if let Some(ref left) = self.left {
            path.push_str("->");
            left.all_paths(path, paths);
        }
        if let Some(ref right) = self.right {
            path.push_str("->");
            right.all_paths(path, paths);
        }
    }
}

/***impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Box<Node>>> {

    }
}***/

#[cfg(test)]
mod tests {
    use super::Node;
    //#[test]
    fn test_tree_node_1() {
        let mut tree_node = Node::new(1);
        tree_node.insert(8);
        tree_node.insert(100);
        tree_node.insert(-1);
        tree_node.insert(50);
        tree_node.insert(200);
        tree_node.insert(300);
        tree_node.insert(-500);
        println!("Height {}", tree_node.height());

        assert_eq!(tree_node.exists(1), true);
        assert_eq!(tree_node.exists(50), true);
        assert_eq!(tree_node.exists(100), true);
        assert_eq!(tree_node.exists(8), true);
        assert_eq!(tree_node.exists(9), false);
        let node = tree_node.find(100);
        assert!(node.is_some());
        println!();
        tree_node.in_order();
        println!();
        tree_node.reverse_in_order();
        println!();
        println!("Min = {:?}", tree_node.min());
        println!("Max = {:?}", tree_node.max());

        let mut result = vec![];
        let mut path = String::new();
        tree_node.all_paths(&mut path, &mut result);
        println!("Result = {:?}", result);
        let post_delete = tree_node.delete(&-500);
        //post_delete.unwrap().in_order();
        println!("============");
        tree_node.in_order();
    }
    #[test]
    fn test_tree_node_2() {
        let mut tree_node = Node::new(20);
        tree_node.insert(3);
        tree_node.insert(30);
        tree_node.insert(2);
        tree_node.insert(1);
        tree_node.insert(25);
        tree_node.insert(50);
        tree_node.insert(-400);
        tree_node.insert(10);
        tree_node.in_order();
        println!("Level order");
        tree_node.level_order();
        //let min_node = Node::min_node(tree_node);
        let min_node = tree_node.min_node_match();
        println!("The min node is {:?}", min_node);
        tree_node.level_order();
    }
}
