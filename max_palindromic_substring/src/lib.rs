///Given a string s, return the max palindromic substring

pub fn longest_palindrome(s: String) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut max = String::new();
    let s: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        for j in (i..s.len()).rev() {
            if is_palindrome(&s, i, j) {
                if j - i + 1 > max.len() {
                    max = s[i..j + 1].iter().collect();
                }
            }
        }
    }
    max
}

pub fn is_palindrome(s: &Vec<char>, mut left: usize, mut right: usize) -> bool {
    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;
    #[test]
    fn longest_palindrome_test1() {
        assert_eq!(longest_palindrome(String::from("abc")), String::from("a"));
    }
    #[test]
    fn longest_palindrome_test2() {
        assert_eq!(
            longest_palindrome(String::from("abba")),
            String::from("abba")
        );
    }
}
