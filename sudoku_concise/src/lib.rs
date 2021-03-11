///Determine if a 9 by 9 sudo board is valid. Empty cells are filled with '.' char
use std::collections::HashSet;
pub fn is_valid(board: &Vec<Vec<char>>) -> bool {
    if board.len() != 9 || board[0].len() != 9 {
        return false;
    }
    let rows = &mut HashSet::<String>::with_capacity(9);
    let cols = &mut HashSet::<String>::with_capacity(9);
    let boxes = &mut HashSet::<String>::with_capacity(9);
    for i in 0..9 {
        for j in 0..9 {
            let ch = board[i][j];
            if ch != '.' {
                let row_entry =
                    String::from("row:") + &i.to_string() + &j.to_string() + &ch.to_string();
                let col_entry =
                    String::from("col:") + &j.to_string() + &i.to_string() + &ch.to_string();
                let box_entry = String::from("box:")
                    + &(i / 3).to_string()
                    + &(j / 3).to_string()
                    + &ch.to_string();
                if !rows.insert(row_entry) || !cols.insert(col_entry) || !boxes.insert(box_entry) {
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
