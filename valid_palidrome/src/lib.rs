///680. Valid Palindrome II
///Given a non-empty string s, you may delete at most one character.
///Judge whether you can make it a palindrome.

pub fn valid_palindrome(s: String) -> bool {
    if s.len() == 0 || s.len() == 1 {
        return true;
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    let s: Vec<char> = s.chars().collect();
    while i < j {
        if s[i] != s[j] {
            return helper(&s, i + 1, j) || helper(&s, i, j - 1);
        }
        i += 1;
        j -= 1;
    }
    return true;
}
fn helper(s: &Vec<char>, ii: usize, jj: usize) -> bool {
    let mut i = ii;
    let mut j = jj;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
#[cfg(test)]
mod tests {
    use super::valid_palindrome;
    #[test]
    fn valid_palindrome_test1() {
        assert_eq!(valid_palindrome("aba".to_string()), true);
    }
    #[test]
    fn valid_palindrome_test2() {
        assert_eq!(valid_palindrome("abca".to_string()), true);
    }
}
