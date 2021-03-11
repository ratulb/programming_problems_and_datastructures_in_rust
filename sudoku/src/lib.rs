///Determine if a 9 by 9 sudo board is valid. Empty cells are filled with '.' char
use std::collections::HashMap;
pub fn is_valid(board: &Vec<Vec<char>>) -> bool {
    if board.len() != 9 || board[0].len() != 9 {
        return false;
    }
    let rows = &mut vec![HashMap::<u32, u32>::with_capacity(9); 9];
    let cols = &mut vec![HashMap::<u32, u32>::with_capacity(9); 9];
    let boxes = &mut vec![HashMap::<u32, u32>::with_capacity(9); 9];
    for i in 0..9 {
        for j in 0..9 {
            let ch = board[i][j];
            if ch != '.' {
                let n = ch as u32 - '0' as u32;
                let box_index = (i / 3) * 3 + j / 3;
                *rows[i].entry(n).or_insert(0) += 1;
                *cols[j].entry(n).or_insert(0) += 1;
                *boxes[box_index].entry(n).or_insert(0) += 1;

                if rows[i][&n] > 1 || cols[j][&n] > 1 || boxes[box_index][&n] > 1 {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_valid;
    #[test]
    fn validate() {
        assert_eq!(is_valid(&vec![]), false);
    }
    #[test]
    fn validate_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid(&board), true);
    }
    #[test]
    fn invalidate_invalid_sudoku() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid(&board), false);
    }
}
