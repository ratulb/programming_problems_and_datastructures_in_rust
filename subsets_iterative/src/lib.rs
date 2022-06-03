///Find all unique subsets of a set with unique elements
///Iterative algorithm

pub fn subsets(arr: &[i32]) -> Vec<Vec<i32>> {
    if arr.len() == 0 {
        return vec![];
    }
    let mut subsets = Vec::<Vec<i32>>::with_capacity(1 << arr.len());
    subsets.push(vec![]);
    for elem in arr {
        let size = subsets.len();
        for i in 0..size {
            let subset = &subsets[i];
            let mut new_subset = subset.to_vec();
            new_subset.push(*elem);
            subsets.push(new_subset);
        }
    }
    subsets
}

#[cfg(test)]
mod tests {
    use super::subsets;
    #[test]
    fn test_subsets() {
        assert_eq!(
            subsets(&[1, 2, 3]),
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
