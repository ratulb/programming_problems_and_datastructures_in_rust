///Run-length encoding (RLE) compression offers a fast way to do efficient on-the-fly
///compression and decompression of strings. The idea is simple—encode successive
///repeated characters by the repetition count and the character. For example, the RLE
///of "aaaabcccaa" is "4alb3c2a". The decoding of "3e4f2e" returns "eeeffffee".
use std::char::from_digit;

pub fn encode(clear_text: &String) -> String {
    if clear_text.len() == 0 {
        return clear_text.to_string();
    }
    let mut encoded = String::new();
    let chars: Vec<char> = clear_text.chars().collect();
    let mut repeat_count = 1;
    for i in 1..clear_text.len() {
        if chars[i - 1] != chars[i] {
            encoded.push(from_digit(repeat_count, 10).unwrap());
            encoded.push(chars[i - 1]);
            repeat_count = 1;
        } else {
            repeat_count += 1;
        }
    }
    if clear_text.len() == 1 {
        encoded.push_str("1");
        encoded.push_str(&clear_text[..]);
        return encoded;
    }
    encoded.push(from_digit(repeat_count, 10).unwrap());
    encoded.push(chars[clear_text.len() - 1]);
    encoded
}

pub fn decode(cipher_text: &String) -> String {
    if cipher_text.len() == 0 {
        return cipher_text.to_string();
    }
    let mut repeat_count = 0;
    let mut clear_text = String::new();
    let chars: Vec<char> = cipher_text.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_digit(10) {
            repeat_count = repeat_count * 10 + chars[i].to_digit(10).unwrap();
        } else {
            while repeat_count > 0 {
                clear_text.push(chars[i]);
                repeat_count -= 1;
            }
        }
    }
    clear_text
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::encode;
    #[test]
    fn test_encoding_1() {
        assert_eq!(encode(&String::from("aaaabcccaa")), "4a1b3c2a");
    }
    #[test]
    fn test_encoding_2() {
        assert_eq!(encode(&String::from("eeeffffee")), "3e4f2e");
    }
    #[test]
    fn test_decoding_1() {
        assert_eq!(decode(&String::from("4a1b3c2a")), "aaaabcccaa");
    }
    #[test]
    fn test_decoding_2() {
        assert_eq!(decode(&String::from("3e4f2e")), "eeeffffee");
    }
}
