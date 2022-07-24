# Substring with Concatenation of All Words

Given a string s and an array of strings words of the same length. Return all starting indices of substring(s) in s that is a concatenation of each word in words exactly once, in any order, and without any intervening characters.

The answer can be in any order.

Example 1:

Input: s = "barfoothefoobarman", words = ["foo","bar"]
Output: [0,9]
Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively
The output order does not matter, returning [9,0] is fine too.

Example 2:

Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]

Output: [12]

Ingnore the duplicate search words above.

Example 3:

Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]

Output: [6,9,12]


### Solution explanation: We use a [double ended queue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) for this solution. We keep iterating through the given input string for chuncks 
of words equal in length to the search key words' and keep pushing them into the queue. When we the 
queue size becomes equal to the search key words' array size, we add the index of the first chunck 
to the result vector and delete all entries but the last - because that could be the beginning of 
another substring. If we encounter duplicate immediately after a previous chunck - we clear queue 
and add just only the current chunck. We also check for previous duplicates - and remove them if any. 
### Following is the implementation:
```rust, ignore
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
    //Get rid of duplicate search words
    let words = HashSet::<String>::from_iter(words);
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
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/substring_concatenation/src/lib.rs)
