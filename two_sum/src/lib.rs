
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

#[cfg(test)]
mod tests {
    use super::two_sum;
    #[test]
    fn test_two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_two_sum_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }
}
