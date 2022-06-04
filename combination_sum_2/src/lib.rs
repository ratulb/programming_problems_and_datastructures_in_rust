/***
 Given a collection of candidate numbers (candidates) and a target number (target), 
 find all unique combinations in candidates where the candidate numbers sum to
 target.

 Each number in candidates may only be used once in the combination.
***/

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut candidates = candidates;
    candidates.sort();
    find_combinations(&candidates, target, 0, &mut result, &mut current);
    result
}

fn find_combinations(
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
    for i in index..candidates.len() {
        if i == index || candidates[i] != candidates[i - 1] {
            if candidates[i] <= target {
                current.push(candidates[i]);
                find_combinations(candidates, target - candidates[i], i + 1, result, current);
                current.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::combination_sum2;
    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = combination_sum2(candidates, target);
        assert_eq!(
            result,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );

        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let result = combination_sum2(candidates, target);
        assert_eq!(result, vec![vec![1, 2, 2], vec![5]]);
    }
}
