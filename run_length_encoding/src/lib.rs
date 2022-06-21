/***
Run-length encoding is a fast and simple method of encoding strings. The basic idea is to represent repeated successive characters as a single count and character. For example, the string "AAAABBBCCDAA" would be encoded as "4A3B2CD2A" - Avoid adding 1 if the occurence is onlyonce.

Implement run-length encoding and decoding. You can assume the string to be encoded have no digits and consists solely of alphabetic characters. You can assume the string to be decoded is valid.
***/

pub fn encode(s: &str) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut result = String::with_capacity(s.len()); //Avoid re-allocation
    let chars: Vec<_> = s.chars().collect();
    let mut i = 0;
    while i < s.len() {
        let ch = chars[i];
        let mut j = i;
        while j < s.len() && chars[j] == ch {
            j += 1;
        }
        if j - i > 1 {
            result.push_str(&(j - i).to_string());
        }
        result.push_str(&ch.to_string());
        i = j;
    }
    result
}

pub fn decode(s: &str) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut result = String::with_capacity(s.len() * 3); //Avoid re-allocation
    let chars: Vec<_> = s.chars().collect();
    let mut i = 0;
    let mut count = 0;
    while i < s.len() {
        let ch = chars[i];
        if ch.is_ascii_digit() {
            let mut j = i;
            while j < s.len() && chars[j].is_ascii_digit() {
                count = count * 10 + chars[j].to_digit(10).unwrap();
                j += 1;
            }
            result.push_str(&chars[j].to_string().repeat(count as usize));
            count = 0;
            i = j;
        } else {
            result.push(ch);
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::encode;
    #[test]
    fn test_rle_encode() {
        assert_eq!(encode("aaaabcccaa"), String::from("4ab3c2a"));
        assert_eq!(encode("eeeffffee"), String::from("3e4f2e"));
    }

    #[test]
    fn test_rle_decode() {
        assert_eq!(decode("4ab3c2a"), String::from("aaaabcccaa"));
        assert_eq!(decode("3e4f2e"), String::from("eeeffffee"));
    }
}
