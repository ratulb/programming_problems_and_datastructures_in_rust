/***
 * https://leetcode.com/problems/unique-paths/
 * A robot is located at the top-left corner of a m x n grid
 * The robot can only move either down or right at any point in time.
 * The robot is trying to reach the bottom-right corner of the grid
 * How many possible unique paths are there?
 ***/

pub fn unique_paths(m: usize, n: usize) -> i32 {
  let mut grid = vec![vec![0; n]; m];
  for i in 0..m {
    grid[i][0] = 1;
  }
  for j in 0..n {
    grid[0][j] = 1;
  }
  for i in 1..m {
    for j in 1..n {
      grid[i][j] = grid[i-1][j] + grid[i][j-1];
    }
  }

  /***for i in 0..m {
      println!("{:?}", grid[i]);
  }***/
  grid[m-1][n-1]

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_grid() {
        assert_eq!(unique_paths(3,7), 28);
        assert_eq!(unique_paths(3,3), 6);
    }
}
