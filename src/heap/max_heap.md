# Max heap

In max heap - parent element is always bigger than its children. When we insert an element to the
heap we insert it at right most postion of the underlying `Vec` data structure and comapre it with
its parent and move it up as long as it is bigger than its parent.

While removing - we take out the top element - replace it with the right most element. Next we,
look at its children to check if it is smaller than them. We swap parent with the bigger one of its
children and repeat the process til parent is no longer smaller than its children.

### Following is the entire code for max heap:
```rust, ignore
/***
 * Implement a max heap data structure
 ***/

#[derive(Debug)]
pub struct MaxHeap<T: Ord> {
    elements: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap {
            elements: Vec::new(),
        }
    }
    //Allocate memory upfront
    pub fn with_capacity(capacity: usize) -> Self {
        MaxHeap {
            elements: Vec::with_capacity(capacity),
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

    //If the newly inserted element (at the last index) - is bigger than its parent,
    //swap parent and the element. Go to parent index position, continue the process
    //until element > parent condition does not hold
    fn heapify_up(&mut self) {
        let mut index = self.elements.len() - 1;
        while Self::has_parent(index) && self.parent(index) < self.elements.get(index) {
            let parent_index = Self::get_parent_index(index).unwrap();
            self.elements.swap(parent_index, index);
            index = parent_index;
        }
    }

    //Takeout the element at the top and replace it with the last
    //element and heapify down
    //Return the element taken out
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

    //Start at the top index element. If it is bigger than either of its children
    //we are done. Otherwise, bring top element down. Continue down.
    fn heapify_down(&mut self) {
        let mut index = 0;
        while self.has_left_child(index) {
            let mut bigger_child_index = Self::left_child_index(index);
            if self.has_right_child(index) && self.right_child(index) > self.left_child(index) {
                bigger_child_index = Self::right_child_index(index);
            }
            if self.elements[index] > self.elements[bigger_child_index] {
                break;
            } else {
                self.elements.swap(index, bigger_child_index);
            }
            index = bigger_child_index;
        }
    }
}
```


