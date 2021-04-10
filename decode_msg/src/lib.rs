///Decode a message that can be encoded by using numbers  1 to 26
///The numbers denote characters A to Z

pub fn decode(msg: String) -> usize {
    fn decode(msg: &String, index: usize) -> usize {
        if index == msg.len() {
            1
        } else if msg.chars().nth(index) == Some('0') {
            0
        } else if index + 2 <= msg.len() && 26 >= msg[index..index + 2].parse::<u32>().unwrap() {
            decode(msg, index+1) + decode(msg, index + 2)
        } else {
            decode(msg, index+1)
        }
    }
    decode(&msg, 0)
}

#[cfg(test)]
mod tests {
    use super::decode;
    #[test]
    fn decode_test_1() {
        assert_eq!(decode(String::from("1202")), 1);
    }
    #[test]
    fn decode_test_2() {
        assert_eq!(decode(String::from("10")), 1);
    }
    #[test]
    fn decode_test_3() {
        assert_eq!(decode(String::from("123")), 3);
    }
}
