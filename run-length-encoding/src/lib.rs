///Run-length encoding (RLE) compression offers a fast way to do efficient on-the-fly
///compression and decompression of strings. The idea is simple—encode successive
///repeated characters by the repetition count and the character. For example, the RLE
///of "aaaabcccaa" is "4alb3c2a". The decoding of "3e4f2e" returns "eeeffffee".
use std::char::from_digit;

pub fn encode(s: &str) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let mut result = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut count = 1;
    for i in 1..s.len() {
        if chars[i - 1] != chars[i] {
            result.push(from_digit(count, 10).unwrap());
            result.push(chars[i - 1]);
            count = 1;
        } else {
            count += 1;
        }
    }
    if s.len() == 1 {
        result.push_str("1");
        result.push_str(&s[..]);
        return result;
    }
    result.push(from_digit(count, 10).unwrap());
    result.push(chars[s.len() - 1]);
    result
}

pub fn decode(s: &str) -> String {
    if s.len() == 0 {
        return s.to_string();
    }
    let mut count = 0;
    let mut result = String::with_capacity(s.len() * 3);//Try avoiding re-allocation
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_digit(10) {
            count = count * 10 + chars[i].to_digit(10).unwrap();
        } else {
            while count > 0 {
                result.push(chars[i]);
                count -= 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::encode;
    #[test]
    fn test_encoding_1() {
        assert_eq!(encode("aaaabcccaa"), "4a1b3c2a");
    }
    #[test]
    fn test_encoding_2() {
        assert_eq!(encode("eeeffffee"), "3e4f2e");
    }
    #[test]
    fn test_decoding_1() {
        assert_eq!(decode("4a1b3c2a"), "aaaabcccaa");
    }
    #[test]
    fn test_decoding_2() {
        assert_eq!(decode("3e4f2e"), "eeeffffee");
    }
}
