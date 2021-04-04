//Merge k-sorted array

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn merge(arrays: &[&[i32]]) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    let mut size = 0;
    for i in 0..arrays.len() {
        size += arrays[i].len();
        if arrays[i].len() > 0 {
            let elem = Elem::new(0, i, arrays[i][0]);
            heap.push(Reverse(elem));
        }
    }
    let mut result: Vec<i32> = Vec::with_capacity(size);
    while !heap.is_empty() {
        let elem = heap.pop().unwrap().0;
        let index = elem.index;
        let array = elem.array;
        let value = elem.value;
        if index + 1 < arrays[array].len() {
            heap.push(Reverse(Elem::new(
                index + 1,
                array,
                arrays[array][index + 1],
            )));
        }
        result.push(value);
    }
    result
}

struct Elem {
    index: usize,
    array: usize,
    value: i32,
}

impl Elem {
    pub fn new(index: usize, array: usize, value: i32) -> Self {
        Elem {
            index,
            array,
            value,
        }
    }
}
impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for Elem {}
use std::cmp::Ordering;
impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

#[cfg(test)]
mod tests {
    use super::merge;
    #[test]
    fn merge_test() {
        assert_eq!(
            merge(&[&[1, 3, 5, 6, 7], &[0, 2, 4], &[2, 4, 6, 8, 9, 10]]),
            vec![0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 9, 10]
        );
    }
}
