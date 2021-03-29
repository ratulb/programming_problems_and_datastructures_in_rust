use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn kth_largest(array: &[i32], k: usize) -> i32 {
    if array.len() == 0 || k == 0 || k >= array.len() {
        return i32::MIN;
    }
    let mut heap = BinaryHeap::with_capacity(k);
    for elem in array {
        if heap.len() < k {
            heap.push(Reverse(elem));
        } else {
            if heap.peek().unwrap().0 < elem {
                heap.pop();
                heap.push(Reverse(elem));
            }
        }
    }
    *heap.pop().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::kth_largest;
    #[test]
    fn test_kth_largest() {
        assert_eq!(kth_largest(&[10, 5, 15, 20, 1, 25, 35, 4, 30], 3), 25);
    }
}
