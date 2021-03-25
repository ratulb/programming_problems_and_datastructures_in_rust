
///Given an m x n grid of characters board and a string word, return true
///if word exists in the grid.

///The word can be constructed from letters of sequentially adjacent cells,
///where adjacent cells are horizontally or vertically neighboring. The same
///letter cell may not be used more than once.

pub fn exists(board: Vec<Vec<char>>, word: String) -> bool {
    if board.len() == 0 || board[0].len() == 0 || word.len() == 0 {
        return false;
    }
    let word: Vec<char> = word.chars().collect();
    let board = &mut board.to_owned();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == word[0] && search_rest(board, &word, i, j, 0) {
                return true;
            }
        }
    }
    false
}

fn search_rest(
    board: &mut Vec<Vec<char>>,
    word: &Vec<char>,
    i: usize,
    j: usize,
    index: usize,
) -> bool {
    if index == word.len() {
        return true;
    }
    if i == board.len() || j == board[i].len() || board[i][j] != word[index] {
        return false;
    }
    //Remember the character at i,j - since it has been matched
    let memorized = board[i][j];
    //Set the character to some improbable value
    board[i][j] = '*';
    let search_down = search_rest(board, word, i + 1, j, index + 1);
    let search_right = search_rest(board, word, i, j + 1, index + 1);
    let search_up = if i > 0 {
        search_rest(board, word, i - 1, j, index + 1)
    } else {
        false
    };
    let search_left = if j > 0 {
        search_rest(board, word, i, j - 1, index + 1)
    } else {
        false
    };
    //Restore the remembered char
    board[i][j] = memorized;
    search_down || search_right || search_up || search_left
}

#[cfg(test)]
mod test {
    use super::exists;
    #[test]
    fn test_exists_1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");
        assert_eq!(exists(board, word), true);
    }
    #[test]
    fn test_exists_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCB");
        assert_eq!(exists(board, word), false);
    }
}
