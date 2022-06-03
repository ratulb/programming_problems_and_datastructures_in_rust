/***
 * Given an array of integers - generate all subsets
 ***/

pub fn subsets(arr: &[i32]) -> Vec<Vec<i32>> {
    if arr.len() == 0 {
        return vec![];
    }
    let mut subsets = Vec::with_capacity(1 << arr.len());
    helper(arr, 0, &mut subsets, &mut Vec::with_capacity(arr.len()));
    subsets
}

fn helper(arr: &[i32], index: usize, subsets: &mut Vec<Vec<i32>>, current: &mut Vec<i32>) {
    if index == arr.len() {
        subsets.push(current.clone());
        return;
    }
    current.push(arr[index]);
    helper(arr, index + 1, subsets, current);
    current.pop();
    helper(arr, index + 1, subsets, current);
}

#[cfg(test)]
mod tests {
    use super::subsets;
    #[test]
    fn test_subsets() {
        let arr = [1, 2, 3];
        let result = subsets(&arr);
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1],
                vec![2, 3],
                vec![2],
                vec![3],
                vec![]
            ]
        );
    }
}
