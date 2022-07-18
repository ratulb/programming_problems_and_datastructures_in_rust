
/// Substring with Concatenation of All Words

use std::collections::HashSet;
use std::collections::VecDeque;

pub fn find_substring(s: String, search_words: Vec<String>) -> Vec<usize> {
    if s.is_empty() || search_words.is_empty() {
        return vec![];
    }
    let result = substring_indices(s, search_words);
    result
}

fn substring_indices(s: String, words: Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    if s.is_empty() || words.is_empty() {
        return result;
    }
    let mut stack: VecDeque<(&str, usize)> = VecDeque::with_capacity(words.len());
    let split_size = words[0].len();
    let words = words
        .into_iter()
        .map(|w| w.to_string())
        .collect::<HashSet<_>>();
    let mut index = 0;
    while index <= s.len() - split_size {
        let chunck = &s[index..index + split_size];
        if words.contains(&chunck.to_string()) {
            if let Some(back) = stack.back() {
                if &back.0 == &chunck {
                    stack.clear();
                    stack.push_back((chunck, index));
                } else {
                    let repeat = stack
                        .iter()
                        .enumerate()
                        .find(|repeat| repeat.1 .0 == chunck);
                    if let Some(repeat) = repeat {
                        for _ in 0..=repeat.0 {
                            stack.pop_front();
                        }
                    }
                    stack.push_back((chunck, index));
                }
            } else {
                stack.push_back((chunck, index));
            }
        } else {
            stack.clear();
        }
        if stack.len() == words.len() {
            let first = stack.pop_front();
            if words.len() >= 2 {
                for _ in 0..words.len() - 2 {
                    stack.pop_front();
                }
            }
            if let Some(first) = first {
                result.push(first.1);
            }
        }
        index += split_size;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn substr_concatenation_test() {
        let s = String::from("barfoofoobarthefoobarman");
        let words = vec![
            "man".to_string(),
            "foo".to_string(),
            "the".to_string(),
            "bar".to_string(),
        ];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![12]);
    }
    #[test]
    fn substr_concatenation_test_1() {
        let s = String::from("bar");
        let words = vec!["bar".to_string()];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![0]);
        let s = String::from("barfoo");
        let words = vec!["foo".to_string()];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![3]);
    }

    #[test]
    
    fn substr_concatenation_test2() {
        let s = String::from("barfoothefoobarman");
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = substring_indices(s, words);
        assert!(result.contains(&0));
        assert!(result.contains(&9));
    }
    #[test]
    #[ignore]
    fn substr_concatenation_test3() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![12]);
    }
    #[test]
    fn substr_concatenation_test4() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec!["best".to_string(), "word".to_string(), "good".to_string()];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![12]);
        let s = String::from("goodbestword");
        let words = vec!["best".to_string(), "word".to_string(), "good".to_string()];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![0]);

    }
    #[test]
    fn substring_indices_windowing_test_1() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let result = substring_indices(s, words);
        assert_eq!(result, vec![12]);
    }
}
