/***
Given an integer array nums and an integer k, return true if there
are two distinct indices i and j in the array such that nums[i] == nums[j]
and abs(i - j) <= k.
***/
use std::collections::HashMap;
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut entries = HashMap::new();
    for i in 0..nums.len() {
        if let Some(index) = entries.get(&nums[i]) {
            if i - index <= k as usize {
                return true;
            } else {
                *entries.entry(&nums[i]).or_insert(i) = i;
            }
        } else {
            *entries.entry(&nums[i]).or_insert(i) = i;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::contains_nearby_duplicate;
    #[test]
    fn test_contains_nearby_duplicate_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let result = contains_nearby_duplicate(nums, k);
        assert!(result);
    }

    #[test]
    fn test_contains_nearby_duplicate_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let result = contains_nearby_duplicate(nums, k);
        assert!(result);
    }
    #[test]
    fn test_contains_nearby_duplicate_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let result = contains_nearby_duplicate(nums, k);
        assert!(!result);
    }
}
