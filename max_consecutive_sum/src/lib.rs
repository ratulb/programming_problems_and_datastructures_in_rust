///Find maximum consecutive sum for given array

pub fn max_consecutive_sum(array: &[i32]) -> i32 {
    if array.len() == 0 {
        return 0;
    }
    let mut curr_max_sum = array[0];
    let mut max_sum = curr_max_sum;
    for i in 1..array.len() {
        curr_max_sum = std::cmp::max(array[i], curr_max_sum + array[i]);
        max_sum = std::cmp::max(max_sum, curr_max_sum);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::max_consecutive_sum;
    #[test]
    fn test_max_consecutive_sum() {
        assert_eq!(
            max_consecutive_sum(&[100, 2, -27, 8, -370, 23, 0, 100, -210, 34, -60]),
            123
        );
    }
}
