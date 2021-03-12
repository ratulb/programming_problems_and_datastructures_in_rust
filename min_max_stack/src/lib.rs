use std::cmp::max;
///A min max stack
use std::cmp::min;

pub struct Stack<T> {
    max_size: usize,
    items: Vec<Entry<T>>,
}
impl<T: Ord + Copy> Stack<T> {
    const MAXSIZE: usize = 10;
    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            max_size: maxsize,
            items: Vec::<Entry<T>>::with_capacity(maxsize),
        }
    }
    pub fn new() -> Self {
        Self::with_capacity(Self::MAXSIZE)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn min(&self) -> Option<&T> {
        match self.items.last() {
            None => None,
            Some(entry) => Some(&entry.min),
        }
    }

    pub fn max(&self) -> Option<&T> {
        match self.items.last() {
            None => None,
            Some(entry) => Some(&entry.max),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.items.last() {
            None => None,
            Some(entry) => Some(&entry.item),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.items.pop() {
            None => None,
            Some(entry) => Some(entry.item),
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.size() == self.max_size {
            return false;
        }
        if self.is_empty() {
            self.items.push(Entry {
                item: item,
                min: item,
                max: item,
            });
        } else {
            let min = min(item, *self.min().unwrap());
            let max = max(item, *self.max().unwrap());
            self.items.push(Entry { item, min, max });
        }
        true
    }
}

struct Entry<T> {
    item: T,
    min: T,
    max: T,
}
#[cfg(test)]
mod tests {
    use super::Stack;
    #[test]
    fn test_stack_min() {
        let mut stack = Stack::new();
        stack.push(100);
        stack.push(-50);
        stack.push(200);
        stack.push(100);
        assert_eq!(stack.min(), Some(&-50));
    }
    #[test]
    fn test_stack_max() {
        let mut stack = Stack::new();
        stack.push(100);
        stack.push(-50);
        stack.push(200);
        stack.push(100);
        assert_eq!(stack.max(), Some(&200));
    }
}
