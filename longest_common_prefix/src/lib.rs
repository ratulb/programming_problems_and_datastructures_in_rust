/***
 * Find the longest prefix from an array of strings
 ***/

pub fn longest_common_prefix(arr: &[&str]) -> String {
    if arr.len() == 0 {
        return String::new();
    }
    let mut result = String::new();
    let mut first = arr[0].chars();
    let mut rests: Vec<_> = arr[1..].iter().map(|s| s.chars()).collect();
    while let Some(ch) = first.next() {
        for i in 0..rests.len() {
            let current = &mut rests[i];
            match current.next() {
                Some(c) if c == ch => continue,
                _ => return result,
            }
        }
        result.push(ch);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_common_prefix_test_1() {
        let arr = ["flower", "flow", "flight"];
        assert_eq!(longest_common_prefix(&arr), String::from("fl"));

        let arr = ["flower1", "flower2", "flower3"];
        assert_eq!(longest_common_prefix(&arr), String::from("flower"));
    }
}
