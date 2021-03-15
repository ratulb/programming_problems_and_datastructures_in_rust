///Find all unique subsets of a set with unique elements
///Recursive algoritm

pub fn all_subsets(array: &[i32]) -> Vec<Vec<i32>> {
    if array.len() == 0 {
        return vec![];
    }
    let mut result = Vec::<Vec<i32>>::with_capacity(2_i32.pow(array.len() as u32) as usize);
    fn generate_subsets(
        array: &[i32],
        index: usize,
        curr_set: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if index == array.len() {
            result.push(curr_set.to_vec());
        } else {
            curr_set.push(array[index]);
            generate_subsets(array, index + 1, curr_set, result);
            curr_set.pop();
            generate_subsets(array, index + 1, curr_set, result);
        }
    }
    generate_subsets(array, 0, &mut Vec::with_capacity(array.len()), &mut result);
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
