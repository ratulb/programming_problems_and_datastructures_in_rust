# Min heap

In a min heap parent is always smaller than its children.

### Following is the code for min heap:
```rust, ignore
/***
  * Implement a min heap data structure
 ***/

  #[derive(Debug)]
  pub struct MinHeap<T: Ord> {
      elements: Vec<T>,
  }

  impl<T: Ord> MinHeap<T> {
      pub fn new(capacity: usize) -> Self {
          MinHeap {
              elements: Vec::with_capacity(capacity),
          }
      }

      fn get_parent_index(index: usize) -> Option<usize> {
          match index {
              0 => None,
              _ => Some((index - 1) / 2),
          }
      }

      fn left_child_index(index: usize) -> usize {
          2 * index + 1
      }

      fn right_child_index(index: usize) -> usize {
          2 * index + 2
      }

      fn has_parent(index: usize) -> bool {
          Self::get_parent_index(index).is_some()
      }

      fn has_left_child(&self, index: usize) -> bool {
          Self::left_child_index(index) < self.elements.len()
      }

      fn has_right_child(&self, index: usize) -> bool {
          Self::right_child_index(index) < self.elements.len()
      }

      fn parent(&self, index: usize) -> Option<&T> {
          match Self::has_parent(index) {
              true => Some(&self.elements[Self::get_parent_index(index).unwrap()]),
              false => None,
          }
      }

      fn left_child(&self, index: usize) -> Option<&T> {
          match self.has_left_child(index) {
              true => Some(&self.elements[Self::left_child_index(index)]),
              false => None,
          }
      }

      fn right_child(&self, index: usize) -> Option<&T> {
          match self.has_right_child(index) {
              true => Some(&self.elements[Self::right_child_index(index)]),
              false => None,
          }
      }

      pub fn insert(&mut self, elem: T) {
          self.elements.push(elem);
          self.heapify_up();
      }
      //Take newly inserted element up as long as it is its parent is bigger
      fn heapify_up(&mut self) {
          let mut index = self.elements.len() - 1;
          while Self::has_parent(index) && self.parent(index) > self.elements.get(index) {
              let parent_index = Self::get_parent_index(index).unwrap();
              self.elements.swap(parent_index, index);
              index = parent_index;
          }
      }

      pub fn remove(&mut self) -> Option<T> {
          match self.elements.len() {
              0 => None,
              _ => {
                  let t = self.elements.swap_remove(0);
                  self.heapify_down();
                  Some(t)
              }
          }
      }
      //Bring parent down comparing it with its children as long as parent is bigger than its
      //children
      fn heapify_down(&mut self) {
          let mut index = 0;
          while self.has_left_child(index) {
              let mut smaller_child_index = Self::left_child_index(index);
              if self.has_right_child(index) && self.right_child(index) < self.left_child(index) {
                  smaller_child_index = Self::right_child_index(index);
              }
              if self.elements[index] < self.elements[smaller_child_index] {
                  break;
              } else {
                  self.elements.swap(index, smaller_child_index);
              }
              index = smaller_child_index;
          }
      }
  }
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/min_heap/src/lib.rs)
