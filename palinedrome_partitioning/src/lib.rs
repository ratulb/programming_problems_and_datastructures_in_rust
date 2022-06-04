/***
 Given a string s, partition s such that every substring of the partition is a palindrome.
 Return all possible palindrome partitioning of s.
 Example 1:

 Input: s = "aab"
 Output: [["a","a","b"],["aa","b"]]
 Example 2:

 Input: s = "a"
 Output: [["a"]]
***/

pub fn partition(s: &str) -> Vec<Vec<String>> {
    if s.len() == 0 {
        return vec![];
    }
    let mut result = Vec::new();
    let mut parts = Vec::new();
    do_partition(s, 0, &mut result, &mut parts);
    result
}

fn do_partition(s: &str, index: usize, result: &mut Vec<Vec<String>>, parts: &mut Vec<String>) {
    if index == s.len() {
        result.push(parts.to_vec());
        return;
    }
    for i in index..s.len() {
        if is_palinedrome(s, index, i) {
            parts.push(String::from(&s[index..i + 1]));
            do_partition(s, i + 1, result, parts);
            parts.pop();
        }
    }
}

fn is_palinedrome(s: &str, start: usize, end: usize) -> bool {
    let mut start = start;
    let mut end = end;
    let s: Vec<_> = s.chars().collect();
    while end >= start {
        if s[start] != s[end] {
            return false;
        }
        start += 1;
        if end == 0 {
            break;
        }
        end -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_palinedrome;
    use super::partition;
    #[test]
    fn test_is_palinedrome() {
        let s = "s";
        assert!(is_palinedrome(s, 0, 0));
    }

    #[test]
    fn test_palinedrome_partition() {
        let s = "aabb";
        assert_eq!(
            partition(s),
            vec![
                vec!["a", "a", "b", "b"],
                vec!["a", "a", "bb"],
                vec!["aa", "b", "b"],
                vec!["aa", "bb"]
            ]
        );
    }
}
