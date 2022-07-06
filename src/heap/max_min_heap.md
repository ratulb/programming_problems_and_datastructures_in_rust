# Max min heap

Here we present a `Heap` struct that can be used either as `min heap` or `max heap` based on how we
instantiate it. `Heap::min` gives a min heap whereas `Heap::max` gives a max heap. Additionally, we
can use the `Heap::with_capacity` to pass in a boolean flag to get a max or min heap.

### Below is the code for the heap struct:
```rust, ignore
/***
 * Implement a heap data structure that can be used as min or max heap
 ***/

pub struct Heap<T: Ord> {
    elements: Vec<T>,
    min: bool,
}

impl<T: Ord> Heap<T> {
    //Creates min heap
    pub fn min() -> Self {
        Heap {
            elements: Vec::new(),
            min: true,
        }
    }
    //Creates a max heap
    pub fn max() -> Self {
        Heap {
            elements: Vec::new(),
            min: false,
        }
    }

    //Allocate memory upfront
    //Creates a max or min heap based on the flag passed in
    pub fn with_capacity(capacity: usize, min: bool) -> Self {
        Heap {
            elements: Vec::with_capacity(capacity),
            min,
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
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
    //If max heap - take the inserted element up as long as it is bigger than its parent
    //If min heap - take the inserted element up as long as it is smaller than its parent
    fn heapify_up(&mut self) {
        let mut index = self.elements.len() - 1;
        while Self::has_parent(index)
            && (!self.min && self.parent(index) < self.elements.get(index)
                || (self.min && self.parent(index) > self.elements.get(index)))
        {
            let parent_index = Self::get_parent_index(index).unwrap();
            self.elements.swap(parent_index, index);
            index = parent_index;
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        match self.is_empty() {
            true => None,
            false => {
                let t = self.elements.swap_remove(0);
                self.heapify_down();
                Some(t)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    pub fn top(&self) -> Option<&T> {
        match self.is_empty() {
            true => None,
            false => self.elements.get(0),
        }
    }

    //If min heap, take the inserted element down as long as it is bigger than its children
    //If max heap, take the inserted element down as long as it is smaller than its children
    fn heapify_down(&mut self) {
        let mut index = 0;
        while self.has_left_child(index) {
            let mut child_index = Self::left_child_index(index);
            if self.has_right_child(index)
                && (!self.min && self.right_child(index) > self.left_child(index)
                    || (self.min && self.right_child(index) < self.left_child(index)))
            {
                child_index = Self::right_child_index(index);
            }
            if !self.min && self.elements[index] > self.elements[child_index] {
                break;
            } else if self.min && self.elements[index] < self.elements[child_index] {
                break;
            } else {
                self.elements.swap(index, child_index);
            }
            index = child_index;
        }
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/heap/src/lib.rs).
