///Find the maximum number of 1 in a binary matrixi

fn max_consecutive_ones(array: &[u8]) -> usize {
    if array.len() == 0 {
        return 0;
    }
    let mut curr_consecutive_ones_count = 0;
    let mut max_consecutive_ones_count = 0;
    for i in 0..array.len() {
        if array[i] == 1 {
            curr_consecutive_ones_count += 1;
            max_consecutive_ones_count =
                std::cmp::max(max_consecutive_ones_count, curr_consecutive_ones_count);
        } else {
            curr_consecutive_ones_count = 0;
        }
    }
    max_consecutive_ones_count
}

#[cfg(test)]
mod tests {
    use super::max_consecutive_ones;
    #[test]
    fn test_max_consecutive_ones() {
        assert_eq!(
            max_consecutive_ones(&[0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1]),
            4
        );
    }
}
