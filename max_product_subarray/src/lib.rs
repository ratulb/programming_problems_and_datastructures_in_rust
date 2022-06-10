/***
Given an integer array nums, find a contiguous non-empty subarray within the array
that has the largest product, and return the product.
***/
//o(n) solution
use std::cmp::max;
use std::cmp::min;
pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut max_product = *nums.iter().max().unwrap();
    let mut current_max = 1;
    let mut current_min = 1;
    for num in nums {
        if num == 0 {
            current_max = 1;
            current_min = 1;
            continue;
        }
        let temp_current_max = current_max;
        current_max = max(max(current_max * num, current_min * num), num);
        current_min = min(min(temp_current_max * num, current_min * num), num);
        max_product = max(max_product, current_max);
    }
    max_product
}

//o(n*n) solution
pub fn max_product_1(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut max_product = *nums.iter().max().unwrap();
    for i in 0..nums.len() {
        let mut product = 1;
        for j in i..nums.len() {
            if nums[j] == 0 {
                product = 1;
                continue;
            }
            product *= nums[j];
            max_product = std::cmp::max(max_product, product);
        }
    }
    max_product
}

#[cfg(test)]
mod tests {
    use super::max_product;
    use super::max_product_1;
    #[test]
    fn max_product_test() {
        let nums = vec![2, 3, -2, 4];
        let result = max_product(nums);
        assert_eq!(result, 6);

        let nums = vec![-2, 0, -1];
        let result = max_product(nums);
        assert_eq!(result, 0);

        let nums = vec![-2, 0, -1, 10, -1, 20, -1];
        let result = max_product(nums);
        assert_eq!(result, 200);
    }

    #[test]
    fn max_product_1_test() {
        let nums = vec![2, 3, -2, 4];
        let result = max_product_1(nums);
        assert_eq!(result, 6);

        let nums = vec![-2, 0, -1];
        let result = max_product_1(nums);
        assert_eq!(result, 0);

        let nums = vec![-2, 0, -1, 10, -1, 20, -1];
        let result = max_product_1(nums);
        assert_eq!(result, 200);
    }
}
