///139. Word Break leetcode
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    if s.len() == 0 || word_dict.is_empty() {
        return false;
    }
    let dictionary: HashSet<&String> = HashSet::from_iter(word_dict.iter());
    word_break_helper(s, &dictionary)
}

fn word_break_helper(s: String, dictionary: &HashSet<&String>) -> bool {
    if s.len() == 0 || dictionary.contains(&s) {
        return true;
    }
    for i in 1..=s.len() {
        let word = &s[0..i];
        if dictionary.contains(&word.to_string())
            && word_break_helper((&s[i..]).to_string(), &dictionary)
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::word_break;

    #[test]
    fn word_break_test1() {
        assert_eq!(
            word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
    }
    #[test]
    fn word_break_test2() {
        assert_eq!(
            word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
    }

    #[test]
    fn word_break_test3() {
        assert_eq!(
            word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }
}
