/***
Given an integer array nums, return an array answer such that answer[i] is equal
to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
integer.

You must write an algorithm that runs in O(n) time and without using the division
operation.
***/
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut prefix = 1;
    for i in 0..nums.len() {
        result[i] = prefix;
        prefix *= nums[i];
    }
    let mut postfix = 1;
    for i in (0..nums.len()).rev() {
        result[i] *= postfix;
        postfix *= nums[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::product_except_self;
    #[test]
    fn test_product_except_self() {
        let nums = vec![1, 2, 3, 4];
        let result = product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);

        let nums = vec![-1, 1, 0, -3, 3];
        let result = product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
