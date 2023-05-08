//https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/

use std::collections::HashSet;
pub fn max_vowels(s: String, k: i32) -> i32 {
    let k: usize = k.try_into().unwrap();
    if s.len() < k.try_into().unwrap() {
        panic!(
            "IllegalArgumentException - string length {} is shorter than substring length {}",
            s.len(),
            k
        );
    }
    let s = s.chars().collect::<Vec<_>>();
    let vowels = "aeiou".chars().collect::<HashSet<_>>();
    let mut count = 0;
    let mut max_count = 0;
    for i in 0..s.len() {
        if vowels.contains(&s[i]) {
            count = count + 1;
        }
        if i < k {
            continue;
        }
        if vowels.contains(&s[i - k]) {
            count = count - 1;
        }
        max_count = std::cmp::max(count, max_count);
    }
    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = max_vowels("abciiidef".to_string(), 3);
        assert_eq!(result, 3);

        let result = max_vowels("leetcode".to_string(), 3);
        assert_eq!(result, 2);

        let result = max_vowels("aeiou".to_string(), 2);
        assert_eq!(result, 2);
    }
}
