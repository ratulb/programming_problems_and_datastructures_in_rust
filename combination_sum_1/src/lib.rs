/***
 Given an array of distinct integers candidates and a target integer target, return a
 list of all unique combinations of candidates where the chosen numbers sum to target.
 You may return the combinations in any order.

 The same number may be chosen from candidates an unlimited number of times. Two
 combinations are unique if the frequency of at least one of the chosen numbers is
 different.
***/

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    combination_sum_helper(&candidates, target, 0, &mut result, &mut current);
    result
}

fn combination_sum_helper(
    candidates: &Vec<i32>,
    target: i32,
    index: usize,
    result: &mut Vec<Vec<i32>>,
    current: &mut Vec<i32>,
) {
    if target == 0 {
        result.push(current.to_vec());
        return;
    }
    if index == candidates.len() {
        return;
    }
    if candidates[index] <= target {
        current.push(candidates[index]);
        combination_sum_helper(
            candidates,
            target - candidates[index],
            index,
            result,
            current,
        );
        current.pop();
    }
    combination_sum_helper(candidates, target, index + 1, result, current);
}

#[cfg(test)]
mod tests {
    use super::combination_sum;
    #[test]
    fn test_combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = combination_sum(candidates, target);
        assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = combination_sum(candidates, target);
        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }
}
