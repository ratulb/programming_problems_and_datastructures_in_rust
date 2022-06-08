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
        if index == 0 {
            None
        } else {
            Some((index - 1) / 2)
        }
    }

    fn get_left_child_index(index: usize) -> usize {
        2 * index + 1
    }

    fn get_right_child_index(index: usize) -> usize {
        2 * index + 2
    }
    fn has_parent(index: usize) -> bool {
        Self::get_parent_index(index).is_some()
    }

    fn has_left_child(&self, index: usize) -> bool {
        Self::get_left_child_index(index) < self.elements.len()
    }
    fn has_right_child(&self, index: usize) -> bool {
        Self::get_right_child_index(index) < self.elements.len()
    }
    fn parent(&self, index: usize) -> Option<&T> {
        if Self::has_parent(index) {
            Some(&self.elements[Self::get_parent_index(index).unwrap()])
        } else {
            None
        }
    }
    fn left_child(&self, index: usize) -> Option<&T> {
        if self.has_left_child(index) {
            Some(&self.elements[Self::get_left_child_index(index)])
        } else {
            None
        }
    }
    fn right_child(&self, index: usize) -> Option<&T> {
        if self.has_right_child(index) {
            Some(&self.elements[Self::get_right_child_index(index)])
        } else {
            None
        }
    }
    pub fn insert(&mut self, elem: T) {
        self.elements.push(elem);
        self.heapify_up();
    }
    fn heapify_up(&mut self) {
        let mut index = self.elements.len() - 1;
        while Self::has_parent(index) && self.parent(index) > self.elements.get(index) {
            let parent_index = Self::get_parent_index(index).unwrap();
            self.elements.swap(parent_index, index);
            index = parent_index;
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.elements.len() == 0 {
            return None;
        }
        let t = self.elements.swap_remove(0);
        self.heapify_down();
        Some(t)
    }
    fn heapify_down(&mut self) {
        let mut index = 0;
        while self.has_left_child(index) {
            let mut smaller_child_index = Self::get_left_child_index(index);
            if self.has_right_child(index) && self.right_child(index) < self.left_child(index) {
                smaller_child_index = Self::get_right_child_index(index);
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

#[cfg(test)]
mod tests {
    use super::MinHeap;
    #[test]
    fn min_heap_test() {
        let mut min_heap = MinHeap::new(10);
        min_heap.insert(20);
        min_heap.insert(5);
        min_heap.insert(7);
        assert_eq!(min_heap.remove(), Some(5));
        assert_eq!(min_heap.remove(), Some(7));
        assert_eq!(min_heap.remove(), Some(20));
        assert_eq!(min_heap.remove(), None);
        min_heap.insert(7);
        min_heap.insert(5);
        assert_eq!(min_heap.remove(), Some(5));
        assert_eq!(min_heap.remove(), Some(7));
        assert_eq!(min_heap.remove(), None);
    }
}
