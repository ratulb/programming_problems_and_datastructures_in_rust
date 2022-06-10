/***
Given an integer array nums, return the length of the longest strictly
increasing subsequence.
***/

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut result = vec![1; nums.len()];
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[j] < nums[i] {
                result[i] = std::cmp::max(result[i], 1 + result[j]);
            }
        }
    }
    *result.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::length_of_lis;
    #[test]
    fn test_max_subsequence_length() {
        let result = length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
        assert_eq!(result, 4);

        let result = length_of_lis(vec![0, 1, 0, 3, 2, 3]);
        assert_eq!(result, 4);

        let result = length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]);
        assert_eq!(result, 1);
    }
}
