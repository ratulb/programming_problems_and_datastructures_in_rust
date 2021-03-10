///Return the longest substring with all unique characters from the given string
use std::collections::HashSet;

pub fn longest_non_repeating_char_subarray(s: &String) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut seen = HashSet::<char>::new();
    let mut start = 0;
    let mut end = 0;
    let characters: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        seen.clear();
        let mut j = i;
        while j < s.len() && !seen.contains(&characters[j]) {
            seen.insert(characters[j]);
            j += 1;
        }
        if end - start < j - i + 1 {
            start = i;
            end = j;
        }
    }
    String::from(&s[start..end])
}

#[cfg(test)]
mod tests {
    use super::longest_non_repeating_char_subarray;
    #[test]
    fn test_empty_char() {
        assert_eq!(
            longest_non_repeating_char_subarray(&"".to_string()),
            String::new()
        );
    }
    #[test]
    fn test_single_char() {
        assert_eq!(
            longest_non_repeating_char_subarray(&"A".to_string()),
            String::from("A")
        );
    }
    #[test]
    fn test_multiple_char() {
        assert_eq!(
            longest_non_repeating_char_subarray(&"ABC".to_string()),
            String::from("ABC")
        );
    }
    #[test]
    fn test_multiple_mixed_char() {
        assert_eq!(
            longest_non_repeating_char_subarray(&"buragohain".to_string()),
            String::from("buragoh")
        );
    }
}
