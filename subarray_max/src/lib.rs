///Return the max element from each sub array of the given array
use std::collections::VecDeque;

pub fn subarray_max(array: &[i32], subarray_size: usize) -> Vec<i32> {
    if array.len() == 0 || subarray_size == 0 {
        return vec![];
    } else if subarray_size >= array.len() {
        return vec![*array.iter().max().unwrap()];
    }
    let mut result = Vec::new();
    let mut queue = VecDeque::<usize>::new();
    let mut prev = 0;
    let mut next = 0;

    while next < array.len() {
        let next_elem = array[next];
        while !queue.is_empty() && array[*queue.back().unwrap()] <= next_elem {
            queue.pop_back();
        }
        queue.push_back(next);
        if next - prev + 1 == subarray_size {
            if !queue.is_empty() && *queue.front().unwrap() < prev {
                queue.pop_front();
            }
            result.push(array[*queue.front().unwrap()]);
            prev += 1;
        }
        next += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::subarray_max;
    #[test]
    fn test_empty_array_input() {
        assert_eq!(subarray_max(&[], 0), vec![]);
    }
    #[test]
    fn test_increasing_array_input() {
        assert_eq!(subarray_max(&[1, 2, 3], 1), vec![1, 2, 3]);
    }
    #[test]
    fn test_increasing_array_size3() {
        assert_eq!(subarray_max(&[1, 2, 3], 3), vec![3]);
    }
    #[test]
    fn test_mixed_array_size3() {
        assert_eq!(
            subarray_max(&[3, 1, 0, 2, 3, 19, 4, -6, 2], 3),
            vec![3, 2, 3, 19, 19, 19, 4]
        );
    }
}
