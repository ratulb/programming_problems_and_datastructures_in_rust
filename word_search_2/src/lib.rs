/***
 Given an m x n board of characters and a list of strings words, return all words
 on the board.

 Each word must be constructed from letters of sequentially adjacent cells,
 where adjacent cells are horizontally or vertically neighboring. The same letter
 cell may not be used more than once in a word. This implementation returns duplicates.
***/

use std::collections::HashMap;
use std::collections::HashSet;

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut root = TrieNode::new();
    for word in &words {
        root.add_word(&word);
    }
    let mut result = Vec::with_capacity(words.len());
    let mut seen = HashSet::new();
    let rows = board.len() as i32;
    let cols = board[0].len() as i32;
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            dfs(
                &board,
                &mut result,
                &mut seen,
                &mut String::new(),
                row as i32,
                col as i32,
                rows,
                cols,
                &root,
            );
        }
    }
    result
}

fn dfs(
    board: &Vec<Vec<char>>,
    result: &mut Vec<String>,
    seen: &mut HashSet<(i32, i32)>,
    word: &mut String,
    row: i32,
    col: i32,
    rows: i32,
    cols: i32,
    mut node: &TrieNode,
) {
    let cell = (row, col);
    if row < 0
        || row == rows
        || col < 0
        || col == cols
        || seen.get(&cell).is_some()
        || !node
            .children
            .contains_key(&board[row as usize][col as usize])
    {
        return;
    }

    seen.insert(cell.clone());
    let ch = board[row as usize][col as usize];
    node = node.children.get(&ch).unwrap();
    word.push(ch);
    if node.is_word {
        result.push(word.to_string());
    }
    dfs(board, result, seen, word, row + 1, col, rows, cols, node);
    dfs(board, result, seen, word, row - 1, col, rows, cols, node);
    dfs(board, result, seen, word, row, col + 1, rows, cols, node);
    dfs(board, result, seen, word, row, col - 1, rows, cols, node);

    seen.remove(&cell);
}

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
    fn add_word(&mut self, word: &str) {
        let mut current = self;
        let mut chars = word.chars();
        while let Some(c) = chars.next() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.is_word = true;
    }
}

#[cfg(test)]
mod tests {
    use super::find_words;
    #[test]
    fn test_find_words() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            String::from("oath"),
            String::from("pea"),
            String::from("eat"),
            String::from("rain"),
        ];
        let result = find_words(board, words);
        assert_eq!(result, vec![String::from("oath"), String::from("eat")]);

        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec![String::from("abcb")];
        let result = find_words(board, words);
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }
}
