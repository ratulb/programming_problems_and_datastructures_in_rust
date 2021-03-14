///Count the number of subsets whose element sum equals a given target

pub fn count_subsets(array: &[u32], target: u32) -> u32 {
    if array.len() == 0 {
        return 0;
    }
    fn count(array: &[u32], target: u32, index: usize) -> u32 {
        if target == 0 {
            return 1;
        } else if index == 0 && array[index] == target {
            return 1;
        } else if index > 0 {
            if array[index] > target {
                return count(array, target, index - 1);
            } else {
                let count1 = count(array, target, index - 1);
                let count2 = if target >= array[index] {
                    count(array, target - array[index], index - 1)
                } else {
                    0
                };
                return count1 + count2;
            }
        } else {
            return 0;
        }
    }
    count(array, target, array.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::count_subsets;
    #[test]
    fn test_count_subsets_1() {
        assert_eq!(count_subsets(&[2, 4, 6, 10], 16), 2);
    }
    #[test]
    fn test_count_subsets_2() {
        assert_eq!(count_subsets(&[2, 2, 2, 2], 2), 4);
    }
    #[test]
    fn test_count_subsets_3() {
        assert_eq!(count_subsets(&[1, 2, 3], 6), 1);
    }
    #[test]
    fn test_count_subsets_4() {
        assert_eq!(count_subsets(&[1], 1), 1);
    }
}
