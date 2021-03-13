///Leetcode 64. Minimum Path Sum
///Given a m x n grid filled with non-negative numbers, find a path from top left to bottom
///right, which minimizes the sum of all numbers along its path.
///You can only move either down or right at any point in time.
use std::cmp;
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 {
        return 0;
    }
    let mut dp = vec![vec![0; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i == 0 && j == 0 {
                dp[0][0] = grid[0][0];
            } else if i == 0 && j > 0 {
                dp[0][j] = dp[0][j - 1] + grid[0][j];
            } else if j == 0 && i > 0 {
                dp[i][0] = dp[i - 1][0] + grid[i][0];
            } else if i > 0 && j > 0 {
                dp[i][j] = grid[i][j] + cmp::min(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[dp.len() - 1][dp[0].len() - 1]
}
#[cfg(test)]
mod tests {
    use super::min_path_sum;
    #[test]
    fn test_minimum_path_sum1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(grid), 7);
    }
    #[test]
    fn test_minimum_path_sum2() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(min_path_sum(grid), 5);
    }
    #[test]
    fn test_minimum_path_sum3() {
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(min_path_sum(grid), 0);
    }
}
