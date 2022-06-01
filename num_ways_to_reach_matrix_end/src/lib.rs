/***
 There is an N by M matrix of zeroes. Given N and M, write a function to count the number of ways of starting at the top-left corner and getting to the bottom-right corner. You can only move right or down.

For example, given a 2 by 2 matrix, you should return 2, since there are two ways to get to the bottom-right:

Right, then down
Down, then right
Given a 5 by 5 matrix, there are 70 ways to get to the bottom-right.

***/

pub fn num_ways_to_reach_matrix_end(rows: usize, cols: usize) -> usize {
    if rows == 0 || cols == 0 {
        return 0;
    }
    let mut matrix = vec![vec![0; cols]; rows];
    //We can reach any cell in top most row only one way i.e. going right
    //Hence fill top most row cells with 1
    for j in 0..cols {
        matrix[0][j] = 1;
    }
    //We can reach any cell in the first column only one way i.e. going down
    //starting at the top left corner
    //Hence fill the first column cells with 1
    for i in 0..rows {
        matrix[i][0] = 1;
    }
    //Now we fill the rest of the cells. Any of the rest of the cells can be reached either
    //comming from the top or from the left the cell.
    for i in 1..rows {
        for j in 1..cols {
            matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1];
        }
    }
    matrix[rows - 1][cols - 1]
}

#[cfg(test)]
mod tests {
    use super::num_ways_to_reach_matrix_end;
    #[test]
    fn test_num_ways_to_reach_end() {
        let rows = 2;
        let cols = 2;
        let num_ways = num_ways_to_reach_matrix_end(rows, cols);
        assert_eq!(num_ways, 2);

        let rows = 5;
        let cols = 5;
        let num_ways = num_ways_to_reach_matrix_end(rows, cols);
        assert_eq!(num_ways, 70);
    }
}
