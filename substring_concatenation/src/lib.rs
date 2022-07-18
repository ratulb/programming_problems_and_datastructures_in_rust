use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

/// Substring with Concatenation of All Words

///You are given a string s and an array of strings words of the same length. Return all starting indices of substring(s) in s that is a concatenation of each word in words exactly once, in any order, and without any intervening characters.

///You can return the answer in any order.

///Example 1:

///Input: s = "barfoothefoobarman", words = ["foo","bar"]
///Output: [0,9]
///Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively///
///The output order does not matter, returning [9,0] is fine too.
///Example 2:

///Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
///Output: []
///Example 3:

///Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
///Output: [6,9,12]
use std::collections::HashSet;
use std::rc::Rc;
pub fn find_substring(s: String, search_words: Vec<String>) -> Vec<usize> {
    if s.is_empty() || search_words.is_empty() {
        return vec![];
    }
    /***let mut words = Words::new(s, search_words[0].len());
    words.init();
    let result = words.substring_indices(&search_words);
    ***/
    let result = substring_indices(s, search_words);
    result
}

#[derive(Debug)]
struct Word {
    word: String,
    index: usize,
}

impl Word {
    fn new(split: &str, index: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            word: String::from(split),
            index,
        }))
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.word.eq(&other.word) && self.index.eq(&other.index)
    }
}

impl Eq for Word {}

#[derive(Debug)]
struct Words {
    s: String,
    split_size: usize,
    word_indices: HashMap<String, Vec<Rc<RefCell<Word>>>>,
    indexed_words: HashMap<usize, Rc<RefCell<Word>>>,
}

impl Words {
    fn new(s: String, split_size: usize) -> Self {
        Self {
            s,
            split_size,
            word_indices: HashMap::new(),
            indexed_words: HashMap::new(),
        }
    }
    fn init(&mut self) {
        let curr = &mut self.s.clone();
        let mut index = 0;
        while !curr.is_empty() {
            let (split, rest) = curr.split_at(self.split_size);
            let word_index = Word::new(split, index);
            let indexed_word = Rc::clone(&word_index);
            self.word_indices
                .entry(split.to_string())
                .or_insert(vec![])
                .push(word_index);
            self.indexed_words.insert(index, indexed_word);
            index += split.len();
            *curr = rest.to_string();
        }
    }

    fn occurences(&self, word: &str) -> Option<std::slice::Iter<'_, Rc<RefCell<Word>>>> {
        self.word_indices
            .get(word)
            .map(|occurences| occurences.iter())
    }
    fn indices(&self, word: &str) -> Vec<usize> {
        match self.occurences(word) {
            None => vec![],
            Some(iter) => iter.map(|word| word.borrow().index).collect::<Vec<_>>(),
        }
    }

    fn word_at_index(&self, index: usize) -> Option<Rc<RefCell<Word>>> {
        self.indexed_words.get(&index).cloned()
    }

    fn next_n_words(&self, index: usize, n: usize) -> Option<Vec<Rc<RefCell<Word>>>> {
        match index + n * self.split_size > self.s.len() {
            true => None,
            false => {
                let mut result = Vec::with_capacity(n);
                (0..n).for_each(|n| {
                    result.push(self.word_at_index(index + n * self.split_size));
                });
                let result = result
                    .iter()
                    .flat_map(|each| each.as_ref().cloned())
                    .collect::<Vec<_>>();
                Some(result)
            }
        }
    }

    fn substring_indices(&self, words: &Vec<String>) -> Vec<usize> {
        let mut result = Vec::new();
        for w in words {
            let indices = self.indices(w);
            for index in indices {
                if index + w.len() * words.len() > self.s.len() {
                    continue;
                }
                let next_n_words = self.next_n_words(index, words.len());
                if let Some(next_words) = next_n_words {
                    let next_words: HashSet<String> =
                        next_words.iter().map(|e| e.borrow().word.clone()).collect();
                    let input_words: HashSet<&String> = HashSet::from_iter(words);
                    let mut contains_all = true;
                    input_words.iter().for_each(|&w| {
                        contains_all = contains_all && next_words.contains(w as &String);
                    });
                    if contains_all {
                        result.push(index);
                    }
                }
            }
        }
        result
    }
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

//Alternate implementation using windowing
fn substring_indices_windowing(s: String, words: Vec<String>) -> Vec<usize> {
    let mut result = vec![];
    if s.is_empty() || words.is_empty() {
        return result;
    }
    let chunck_size = words[0].len();
    let words = words.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
    let mut start = 0;
    let mut end = 0;
    let mut counts = HashMap::new();
    while end <= s.len() - chunck_size * words.len() {
        if end == 0 {
            let mut chuncks = &mut s[0..chunck_size * words.len()].to_string();
            while !chuncks.is_empty() {
                let (prefix, suffix) = chuncks.split_at(chunck_size);
                if words.contains(&prefix) {
                    *counts.entry(prefix.to_string()).or_insert(0) += 1;
                }
                *chuncks = suffix.to_string();
                println!("Segment at start = {:?}", &s[start..end]);
                end += chunck_size * words.len();

                println!("Counts start = {:?}", counts);
                continue;
            }
        }
        let chunck_at_start = &s[start..start + chunck_size];
        if words.contains(&chunck_at_start) {
            let chunck_at_start = chunck_at_start.to_string();
            let count = counts.get(&chunck_at_start).unwrap_or(&0);
            if count > &0 {
                if let Some(value) = counts.get_mut(&chunck_at_start) {
                    *value -= 1;
                }
            }
        }
        println!("Counts = {:?}", counts);

        if end + chunck_size < s.len() {
            let next_chunck = &s[end..chunck_size];
            if words.contains(&next_chunck) {
                *counts.entry(next_chunck.to_string()).or_insert(0) += 1;
            }
            println!("Segment = {:?}", &s[start..end]);
        }

        start += chunck_size;
        end += chunck_size;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    #[test]
    fn test_equality() {
        let s = "This is a testt";
        let split = &s[0..2];
        let word1 = Word::new(split, 10);
        let word2 = Word::new(split, 10);
        let split2 = &s[0..3];
        let word3 = Word::new(split2, 10);
        let word4 = Word::new(split, 11);
        assert_eq!(word1, word2);
        assert_ne!(word1, word3);
        assert_ne!(word1, word4);
    }
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
    #[ignore]
    fn substr_concatenation_test2() {
        let s = String::from("barfoothefoobarman");
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = find_substring(s, words);
        println!("Result = {:?}", result);
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
        println!("Result = {:?}", result);
        assert_eq!(result, vec![12]);
    }
    #[test]
    #[ignore]
    fn substr_concatenation_test4() {
        //let s = String::from("wordgoodgoodgoodbestword");
        let s = String::from("goodbestword");
        let words = vec!["best".to_string(), "word".to_string(), "good".to_string()];
        let result = substring_indices(s, words);
        println!("Result = {:?}", result);
        assert_eq!(result, vec![0]);
    }
    #[test]
    #[ignore]
    fn substring_indices_windowing_test_1() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let result = substring_indices(s, words);
        println!("Result = {:?}", result);
    }
}
