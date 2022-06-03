/***Generate all subsets of an array elemensts via bit manipulation
 ***/

pub fn subsets(arr: &[i32]) -> Vec<Vec<i32>> {
    if arr.len() == 0 {
        return vec![];
    }
    let mut subsets = Vec::with_capacity(1 << arr.len());
    for set in 0..1 << arr.len() {
        let mut subset = Vec::with_capacity(arr.len());
        for index in 0..arr.len() {
            if (set & (1 << index)) >> index == 1 {
                subset.push(arr[index]);
            }
        }
        subsets.push(subset);
    }
    subsets
}

#[cfg(test)]
mod tests {
    use super::subsets;
    #[test]
    fn test_subsets() {
        let arr = [1, 2, 3];
        let subsets = subsets(&arr);
        assert_eq!(
            subsets,
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
