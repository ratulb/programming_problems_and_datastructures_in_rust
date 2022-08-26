# Two sum

Given an array of integers nums and an integer target, return indices of the two
numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not
use the same element twice.

You can return the answer in any order

### Two sum implementation
```rust,ignore
//Two sum implementation

use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..nums.len() {
        if let Some(index) = map.get(&(target - nums[i])) {
            return vec![*index, i as i32];
        } else {
            map.insert(nums[i], i as i32);
        }
    }
    vec![]
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/two_sum/src/lib.rs).
