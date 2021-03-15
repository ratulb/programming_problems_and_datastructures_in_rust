///Find all unique subsets of a set with unique elements
///Iterative algoritm

pub fn all_subsets(array: &[i32]) -> Vec<Vec<i32>> {
    if array.len() == 0 {
        return vec![];
    }
    let mut result = Vec::<Vec<i32>>::with_capacity(2_i32.pow(array.len() as u32) as usize);
    result.push(vec![]);
    for elem in array {
        let curr_size_all_subsets = result.len();
        for i in 0..curr_size_all_subsets {
            let existing_subset = &result[i];
            let mut new_subset_from_existing = existing_subset.to_vec();
            new_subset_from_existing.push(*elem);
            result.push(new_subset_from_existing);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::all_subsets;
    #[test]
    fn test_all_subsets_gen() {
        assert_eq!(
            all_subsets(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
