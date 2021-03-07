pub fn first_negatives(array: &[i32], window_size: usize) -> Vec<i32> {
    if array.len() == 0 {
        return vec![];
    }
    let mut negatives = vec![];
    let mut first_negatives = vec![];
    let mut window_start = 0;

    for window_end in 0..array.len() {
        if array[window_end] < 0 {
            negatives.push(array[window_end]);
        }
        if window_end - window_start + 1 == window_size {
            if !negatives.is_empty() {
                first_negatives.push(negatives.remove(0));
            }
            window_start += 1;
        }
    }
    first_negatives
}

#[cfg(test)]
mod tests {
    use super::first_negatives;
    #[test]
    fn all_negatives() {
        let array = [-1, -2, -3];
        assert_eq!(first_negatives(&array, 1), vec![-1, -2, -3]);
        let array = [-1, -2, -3];
        assert_eq!(first_negatives(&array, 2), vec![-1, -2]);
    }
    #[test]
    fn mixed_negatives() {
        let array = [1, 2, -3];
        assert_eq!(first_negatives(&array, 3), vec![-3]);
        let array = [1, -2, 3, 4, 5, 6];
        assert_eq!(first_negatives(&array, 3), vec![-2]);
        let array = [1, -2, -3, 4, 5, 6];
        assert_eq!(first_negatives(&array, 3), vec![-2, -3]);
    }
}
