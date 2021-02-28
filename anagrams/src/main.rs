use std::env;

fn main() {
    let words: Vec<String> = env::args().collect();
    let words = (words[1].to_owned(), words[2].to_owned());
    let is_anagram = check_anagrams(&words);
    println!("Anagrams {:?} {}", words, is_anagram);
}
pub fn check_anagrams(words: &(String, String)) -> bool {
    let mut word_0: Vec<char> = words.0.chars().collect();
    let mut word_1: Vec<char> = words.1.chars().collect();
    word_0.sort_by(|a,b| a.cmp(b));
    word_1.sort_by(|a,b| a.cmp(b));
    word_0 == word_1      
}

