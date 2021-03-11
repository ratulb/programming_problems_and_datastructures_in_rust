///Find the longest common prefix among array of strings

pub fn longest_common_prefix(strings: &[String]) -> String {
    if strings.len() == 0 {
        return String::new();
    }
    let mut longest_common_prefix = String::new();
    let mut char_vecs = Vec::<Vec<char>>::with_capacity(strings.len());
    strings
        .iter()
        .for_each(|string| char_vecs.push(string.chars().collect()));
    let mut i = 0;
    while i < strings[0].len() {
        let ch = char_vecs[0][i];
        for j in 1..strings.len() {
            if i < strings[j].len() && ch == char_vecs[j][i] {
                continue;
            } else {
                return longest_common_prefix;
            }
        }
        longest_common_prefix.push(ch);
        i += 1;
    }
    longest_common_prefix
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(&[
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
        assert_eq!(
            longest_common_prefix(&[
                String::from("flower"),
                String::from("flower"),
                String::from("flower")
            ]),
            String::from("flower")
        );
    }
}
