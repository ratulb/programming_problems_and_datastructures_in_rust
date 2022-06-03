/***
 * Place N queens in a grid of N * N such that no queen can attack any other queen
 ***/
#![allow(unused_comparisons)]
pub fn place_queens(n: usize) -> Vec<Vec<Vec<u8>>> {
    assert!(n >= 4);
    let mut results = Vec::new();
    let mut board = vec![vec![0; n]; n];
    place(0, &mut board, &mut results, n);
    results
}

fn place(col: usize, board: &mut Vec<Vec<u8>>, results: &mut Vec<Vec<Vec<u8>>>, n: usize) {
    if col == n {
        results.push(board.to_vec());
        return;
    }
    for row in 0..n {
        if can_place(row, col, board, n) {
            board[row][col] = 1;
            place(col + 1, board, results, n);
            board[row][col] = 0;
        }
    }
}

fn can_place(row: usize, col: usize, board: &Vec<Vec<u8>>, n: usize) -> bool {
    //Up left diagonal
    let mut r = row;
    let mut c = col;
    while r >= 0 && c >= 0 {
        if board[r][c] == 1 {
            return false;
        }
        if r == 0 || c == 0 {
            break;
        }
        r -= 1;
        c -= 1;
    }
    //Left check
    c = col;
    while c >= 0 {
        if board[row][c] == 1 {
            return false;
        }
        if c == 0 {
            break;
        }
        c -= 1;
    }
    //Left down diagonal check
    r = row;
    c = col;
    while r < n && c >= 0 {
        if board[r][c] == 1 {
            return false;
        }
        if c == 0 {
            break;
        }
        r += 1;
        c -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::place_queens;
    #[test]
    fn test_place_queens() {
        let results = place_queens(4);
        assert_eq!(
            results,
            vec![
                vec![
                    vec![0, 0, 1, 0],
                    vec![1, 0, 0, 0],
                    vec![0, 0, 0, 1],
                    vec![0, 1, 0, 0]
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 0, 0, 1],
                    vec![1, 0, 0, 0],
                    vec![0, 0, 1, 0]
                ]
            ]
        );
    }
}
