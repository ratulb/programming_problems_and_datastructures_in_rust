# Max sub array sum

Given an integer array, find the contiguous subarray (containing at least one number) which has 
the largest sum and return its sum.

### O(n^3) implementation
```rust,ignore
pub fn max_sub_array_2(arr: Vec<i32>) -> i32 {
    match arr.len() {
        0 => 0,
        _ => {
            let mut max_sum = arr[0];
            for i in 0..arr.len() {
                for j in i..arr.len() {
                    let mut sum = arr[i];
                    for k in i + 1..=j {
                        sum += arr[k];
                    }
                    max_sum = max(max_sum, sum);
                }
            }
            max_sum
        }
    }
}
```
### O(n^2) implementation
```rust,ignore
pub fn max_sub_array_1(arr: Vec<i32>) -> i32 {
    match arr.len() {
        0 => 0,
        _ => {
            let mut max_sum = arr[0];
            for i in 0..arr.len() {
                let mut curr = 0;
                for j in i..arr.len() {
                    curr += arr[j];
                    max_sum = max(max_sum, curr);
                }
            }
            max_sum
        }
    }
}
```
### O(n) implementation
```rust, ignore
use std::cmp::max;
pub fn max_sub_array(arr: Vec<i32>) -> i32 {
    match arr.len() {
        0 => 0,
        _ => {
            let mut max_sum = arr[0];
            let mut curr = 0;
            for num in arr {
                if curr < 0 {
                    curr = 0;
                }
                curr += num;
                max_sum = max(max_sum, curr);
            }
            max_sum
        }
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/max_subarray_sum/src/lib.rs)
