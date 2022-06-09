/***
Given an integer array nums, find the contiguous subarray (containing at
least one number) which has the largest sum and return its sum.

A subarray is a contiguous part of an array.
Example 1:

Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.
Example 2:

Input: nums = [1]
Output: 1
Example 3:

Input: nums = [5,4,-1,7,8]
Output: 23
***/
//o(n) solution
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut current_sum = 0;
    for num in nums {
        if current_sum < 0 {
            current_sum = 0;
        }
        current_sum += num;
        max_sum = std::cmp::max(max_sum, current_sum);
    }
    max_sum
}
//o(n*n) solution
pub fn max_sub_array_2(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            max_sum = std::cmp::max(max_sum, sum);
        }
    }
    max_sum
}

//o(n*n*n) solution - brute force
pub fn max_sub_array_3(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            let mut sum = 0;
            for k in i..=j {
                sum += nums[k];
            }
            max_sum = std::cmp::max(max_sum, sum);
        }
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::max_sub_array;
    use super::max_sub_array_2;
    use super::max_sub_array_3;
    #[test]
    fn test_max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array(nums);
        assert_eq!(result, 6);

        let nums = vec![1];
        let result = max_sub_array(nums);
        assert_eq!(result, 1);

        let nums = vec![5, 4, -1, 7, 8];
        let result = max_sub_array(nums);
        assert_eq!(result, 23);
    }
    #[test]
    fn test_max_sub_array_2() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array_2(nums);
        assert_eq!(result, 6);

        let nums = vec![1];
        let result = max_sub_array_2(nums);
        assert_eq!(result, 1);

        let nums = vec![5, 4, -1, 7, 8];
        let result = max_sub_array_2(nums);
        assert_eq!(result, 23);
    }
    #[test]
    fn test_max_sub_array_3() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array_3(nums);
        assert_eq!(result, 6);

        let nums = vec![1];
        let result = max_sub_array_3(nums);
        assert_eq!(result, 1);

        let nums = vec![5, 4, -1, 7, 8];
        let result = max_sub_array_3(nums);
        assert_eq!(result, 23);
    }
}
