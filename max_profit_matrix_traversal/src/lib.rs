/***
 * You are given a 2-d matrix where each cell represents number of coins in that cell. Assuming we start at matrix[0][0], and can only move right or down, find the maximum number of coins you can collect by the bottom right corner.

For example, in this matrix

0 3 1 1
2 0 0 4
1 5 3 1
The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = 12 coins.
***/

pub fn max_profit(matrix: &[&[u8]]) -> usize {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return 0;
    }
    calculate_max(matrix, 0, 0, 0)
}

fn calculate_max(matrix: &[&[u8]], row: usize, col: usize, mut max: usize) -> usize {
    if row == matrix.len() || col == matrix[0].len() {
        return max;
    }
    let max_going_right =
        max + matrix[row][col] as usize + calculate_max(matrix, row, col + 1, max);
    let max_going_down = max + matrix[row][col] as usize + calculate_max(matrix, row + 1, col, max);
    std::cmp::max(max_going_right, max_going_down)
}

#[cfg(test)]
mod tests {
    use crate::max_profit;
    #[test]
    fn assert_max12() {
        let arr: &[&[u8]] = &[
            &[0u8, 3u8, 1u8, 1u8],
            &[2u8, 0u8, 0u8, 4u8],
            &[1u8, 5u8, 3u8, 1u8],
        ];
        let max = max_profit(arr);
        assert_eq!(max_profit(arr), 12);
    }

#[test]
    fn assert_max40() {
        let arr: &[&[u8]] = &[
            &[8u8, 3u8, 1u8, 1u8],
            &[2u8, 8u8, 0u8, 4u8],
            &[1u8, 5u8, 8u8, 8u8],
        ];
        let max = max_profit(arr);
        assert_eq!(max_profit(arr), 40);
    }

}

