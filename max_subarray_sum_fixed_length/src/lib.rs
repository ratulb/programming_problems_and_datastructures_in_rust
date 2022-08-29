use std::cmp::max;

pub fn max_sum(array: &[i32], window_size: usize) -> i32 {
    let mut max_sum = i32::MIN;
    let mut window_sum = 0;
    let mut window_start = 0;
    for window_end in 0..array.len() {
        window_sum += array[window_end];
        if window_end - window_start + 1 == window_size {
            max_sum = max(max_sum, window_sum);
            window_sum -= array[window_start];
            window_start += 1;
        }
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::max_sum;
    #[test]
    fn test_one_step_linear() {
        assert_eq!(max_sum(&[1, 2, 2, 3], 1), 3);
    }
    #[test]
    fn test_two_step_linear() {
        assert_eq!(max_sum(&[1, 2, 2, 3], 2), 5);
    }
    #[test]
    fn test_three_step_random() {
        assert_eq!(max_sum(&[1, 12, 21, 3, 42, 29, 1, -1, 5], 3), 74);
    }
}
