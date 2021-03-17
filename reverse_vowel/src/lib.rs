///Reverse the vowels in a string

const VOWELS: &str = "aeiouAEIOU";

pub fn reverse_vowels(s: &str) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut i = 0;
    let mut j = s.len() - 1;
    let mut chars: Vec<char> = s.chars().collect();
    while i < j {
        while i < j && !is_vowel(chars[i]) {
            i += 1;
        }
        while i < j && !is_vowel(chars[j]) {
            j -= 1;
        }
        let ch = chars[i];
        chars[i] = chars[j];
        chars[j] = ch;
        i += 1;
        j -= 1;
    }
    chars.into_iter().collect()
}
fn is_vowel(ch: char) -> bool {
    match VOWELS.chars().position(|c| c == ch) {
        None => false,
        Some(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_vowels;
    #[test]
    fn test_reverse_vowels() {
        assert_eq!(
            reverse_vowels("Reverse Vowel"),
            String::from("Revorse Vewel")
        );
    }
}
