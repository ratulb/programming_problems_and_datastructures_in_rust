/***
Given an integer array nums, return true if any value appears at least
twice in the array, and return false if every element is distinct.
***/
use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::with_capacity(nums.len());
    for num in nums {
        if set.contains(&num) {
            return true;
        } else {
            set.insert(num);
        }
    }
    false
}

pub fn contains_duplicate_2(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort();
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;
    use super::contains_duplicate_2;
    #[test]
    fn contains_duplicate_1_test_1() {
        let nums = vec![1, 2, 3, 1];
        let result = contains_duplicate(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn contains_duplicate_1_test_2() {
        let nums = vec![1, 2, 3, 4];
        let result = contains_duplicate(nums);
        assert_eq!(result, false);
    }
    #[test]
    fn contains_duplicate_1_test_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = contains_duplicate(nums);
        assert_eq!(result, true);
    }
    #[test]
    fn contains_duplicate_2_test_1() {
        let nums = vec![1, 2, 3, 1];
        let result = contains_duplicate_2(nums);
        assert_eq!(result, true);
    }
    #[test]
    fn contains_duplicate_2_test_2() {
        let nums = vec![1, 2, 3, 4];
        let result = contains_duplicate_2(nums);
        assert_eq!(result, false);
    }
    #[test]
    fn contains_duplicate_2_test_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = contains_duplicate_2(nums);
        assert_eq!(result, true);
    }
}
