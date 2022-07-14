# Merge k sorted arrays

Here we present a simple routine to merge k sorted arrays which might not be of same
lengths. We make use our `Heap` datastructure that we developed earlier. We also, make 
use of tuple struct to maintain the state of each array while we are iterating over them.

### Following is the implementation:
```rust, ignore
//Merge k sorted arrays. Arrays are not of equal lengths
use heap::Heap;

pub fn merge(arrays: &[&[i32]]) -> Vec<i32> {
    let mut heap = Heap::min();
    let mut size = 0;
    for i in 0..arrays.len() {
        size += arrays[i].len();
        if arrays[i].len() > 0 {
            let elem = Elem::new(arrays[i][0], i, 0);
            heap.insert(elem);
        }
    }
    let mut result: Vec<i32> = Vec::with_capacity(size);
    while !heap.is_empty() {
        let elem = heap.remove().unwrap();
        let value = elem.0;
        let array_idx = elem.1;
        let value_idx = elem.2;
        if value_idx + 1 < arrays[array_idx].len() {
            heap.insert(Elem::new(
                arrays[array_idx][value_idx + 1],
                array_idx,
                value_idx + 1,
            ));
        }
        result.push(value);
    }
    result
}
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Elem<T: Ord>(T, usize, usize);

impl<T: Ord> Elem<T> {
    fn new(value: T, array_idx: usize, value_idx: usize) -> Self {
        Elem(value, array_idx, value_idx)
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
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/merge_ksorted_arrays/src/lib.rs)

