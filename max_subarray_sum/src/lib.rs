///Given an integer array nums, find the contiguous subarray (containing at
///least one number) which has the largest sum and return its sum.

//o(n) solution
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
//o(n*n) solution
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
//o(n*n*n) solution - brute force
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
#[cfg(test)]
mod tests {
    use super::max_sub_array;
    use super::max_sub_array_1;
    use super::max_sub_array_2;
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
    fn test_max_sub_array_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array_1(nums);
        assert_eq!(result, 6);

        let nums = vec![1];
        let result = max_sub_array_1(nums);
        assert_eq!(result, 1);

        let nums = vec![5, 4, -1, 7, 8];
        let result = max_sub_array_1(nums);
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
}
