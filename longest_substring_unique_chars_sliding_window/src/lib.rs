///longest substring from a string with a given number of unique characters
use std::collections::HashMap;

pub fn longest_substring(string: &String, unique_char_count: usize) -> String {
    if string.len() == 0 || unique_char_count == 0 {
        return String::new();
    }
    let mut runner = 0;
    let mut follower = 0;
    let mut output_start_index = 0;
    let mut output_end_index = 0;
    let mut char_map = HashMap::new();
    let characters: Vec<char> = string.chars().collect();

    for ch in &characters {
        *char_map.entry(ch).or_insert(0) += 1;
        if char_map.len() == unique_char_count {
            if output_end_index - output_start_index < runner - follower + 1 {
                output_start_index = follower;
                output_end_index = runner;
            }
        }
        if char_map.len() > unique_char_count {
            let following_char = characters[follower];
            *char_map.get_mut(&following_char).unwrap() -= 1;
            if *char_map.get(&following_char).unwrap() == 0 {
                char_map.remove(&following_char);
            }
            follower += 1;
        }
        runner += 1;
    }
    String::from(&string[output_start_index..output_end_index + 1])
}

#[cfg(test)]
mod tests {
    use super::longest_substring;
    #[test]
    fn test_whole_string() {
        assert_eq!(
            longest_substring(&String::from("abc"), 3),
            String::from("abc")
        );
    }
    #[test]
    fn test_one_char() {
        assert_eq!(longest_substring(&String::from("a"), 1), String::from("a"));
    }
    #[test]
    fn test_starting_string() {
        assert_eq!(
            longest_substring(&String::from("abcddd"), 3),
            String::from("bcddd")
        );
    }
    #[test]
    fn test_ending_string() {
        assert_eq!(
            longest_substring(&String::from("aaabcd"), 3),
            String::from("aaabc")
        );
    }
    #[test]
    fn test_mid_string() {
        assert_eq!(
            longest_substring(&String::from("abcccd"), 3),
            String::from("abccc")
        );
    }
}
