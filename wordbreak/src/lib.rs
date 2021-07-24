///Break a string against a dictionary of words
use std::collections::HashSet;
pub fn wordbreak(s: String, set: HashSet<String>) -> Vec<String> {
    if s.is_empty() || set.is_empty() {
        return vec![];
    }
    let mut result = vec![];
    let mut curr_str = String::new();
    let len = s.len();
    break_words(s, &mut result, &mut curr_str, &set, len);
    println!("The result is {:?} ", result);
    result
}

fn break_words(
    s: String,
    list: &mut Vec<String>,
    curr_str: &mut String,
    set: &HashSet<String>,
    index: usize,
) {
    if s.is_empty() {
        list.push(curr_str.to_string().trim_end().to_owned());
        curr_str.clear();
    }
    for idx in 0..=index {
        let slice = &s[..idx];
        if set.contains(slice) {
            if !curr_str.is_empty() {
                curr_str.push(' ');
            }
            curr_str.push_str(slice);
            break_words(s[idx..].to_string(), list, curr_str, set, index - idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wordbreak;
    use std::collections::HashSet;
    #[test]
    fn test_wordbreak() {
        let s = String::from("footballisthegameweplay");
        let mut set = HashSet::<String>::new();
        set.insert("foot".to_owned());
        set.insert("the".to_owned());
        set.insert("is".to_owned());
        set.insert("play".to_owned());
        set.insert("game".to_owned());
        set.insert("we".to_owned());
        set.insert("ball".to_owned());
        set.insert("football".to_owned());
        let result = wordbreak(s, set);
        assert_eq!(
            result,
            vec![
                "foot ball is the game we play",
                "football is the game we play"
            ]
        );
    }
}
