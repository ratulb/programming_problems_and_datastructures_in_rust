use std::cmp::max;
///Given two strings get the maximum length of their subsequene
use std::iter::FromIterator;

pub fn max_subsequence_len(s1: String, s2: String) -> usize {
    if s1.len() == 0 || s2.len() == 0 {
        return 0;
    }
    let mut s1: Vec<char> = s1.chars().collect();
    let mut s2: Vec<char> = s2.chars().collect();
    if s1[s1.len() - 1] == s2[s2.len() - 1] {
        s1.pop();
        s2.pop();
        return 1 + max_subsequence_len(String::from_iter(s1), String::from_iter(s2));
    } else {
        let mut s1_cloned = s1.clone();
        let mut s2_cloned = s2.clone();
        s1_cloned.pop();
        s2_cloned.pop();
        let count1 = max_subsequence_len(String::from_iter(s1_cloned), String::from_iter(s2));
        let count2 = max_subsequence_len(String::from_iter(s1), String::from_iter(s2_cloned));
        return max(count1, count2);
    }
}

#[cfg(test)]
mod tests {
    use super::max_subsequence_len;
    #[test]
    fn test_max_subsequence_len() {
        assert_eq!(
            max_subsequence_len(String::from("BACDB"), String::from("BDCB")),
            3
        );
    }
}
