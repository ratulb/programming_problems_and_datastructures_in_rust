/***
 Given an m x n grid of characters board and a string word, return true if word
 exists in the grid.

 The word can be constructed from letters of sequentially adjacent cells, where
 adjacent cells are horizontally or vertically neighboring. The same letter cell
 may not be used more than once.
***/

use std::collections::HashSet;

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if board.len() == 0 {
        return false;
    }
    let rows = board.len();
    let cols = board[0].len();
    let word: Vec<_> = word.chars().collect();
    let mut path = HashSet::<(i32, i32)>::with_capacity(word.len());
    for row in 0..rows {
        for col in 0..cols {
            if search(
                &board,
                &word,
                row as i32,
                col as i32,
                0,
                &mut path,
                rows as i32,
                cols as i32,
            ) {
                return true;
            }
        }
    }
    false
}

fn search(
    board: &Vec<Vec<char>>,
    word: &Vec<char>,
    row: i32,
    col: i32,
    index: usize,
    path: &mut HashSet<(i32, i32)>,
    rows: i32,
    cols: i32,
) -> bool {
    if index == word.len() {
        return true;
    }
    let cell = (row, col);
    if row >= rows
        || row < 0
        || col >= cols
        || col < 0
        || path.contains(&cell)
        || board[row as usize][col as usize] != word[index]
    {
        return false;
    }
    path.insert(cell.clone());
    let found = search(board, word, row + 1, col, index + 1, path, rows, cols)
        || search(board, word, row - 1, col, index + 1, path, rows, cols)
        || search(board, word, row, col + 1, index + 1, path, rows, cols)
        || search(board, word, row, col - 1, index + 1, path, rows, cols);
    path.remove(&cell);
    found
}

#[cfg(test)]
mod tests {
    use super::exist;
    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");
        let result = exist(board, word);
        assert!(result);

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("SEE");
        let result = exist(board, word);
        assert!(result);

        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCB");
        let result = exist(board, word);
        assert!(!result);
    }
}
