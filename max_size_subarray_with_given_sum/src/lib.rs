pub fn largest_subarray(array: &[i32], target_sum: i32) -> Option<(usize, usize)> {
    if array.len() == 0 {
        return None;
    }
    let mut runner_ending_index = 0;
    let mut follower_index_at_max = 0;
    let mut subarray_max_size = 0;
    let mut follower = 0;
    let mut met_target_sum = false;
    while follower < array.len() {
        let mut running_sum = 0;
        let mut runner = follower;
        while runner < array.len() {
            running_sum += array[runner];
            if running_sum == target_sum {
                met_target_sum = true;
                if runner - follower + 1 > subarray_max_size {
                    subarray_max_size = runner - follower + 1;
                    runner_ending_index = runner;
                    follower_index_at_max = follower;
                }
            }
            runner += 1;
        }
        follower += 1;
    }
    if met_target_sum {
        Some((follower_index_at_max, runner_ending_index))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::largest_subarray;
    #[test]
    fn test_largest_subarray() {
        assert_eq!(
            largest_subarray(&[0, 0, 0, 1, 1, 1, 2, 5, 1, 2, 3], 5),
            Some((0, 6))
        );
    }
    #[test]
    fn test_empty_array() {
        assert_eq!(largest_subarray(&[], 1234), None);
    }
    #[test]
    fn test_zero_sum() {
        assert_eq!(largest_subarray(&[1,-1,1,1,1,1,-1,-1,-1,1,-1,1, -1, 1, -1, -1], 0), Some((0,15)));
    }
}
