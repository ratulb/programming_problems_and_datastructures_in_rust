/***
 * Find the longest prefix from an array of strings
 ***/

pub fn longest_common_prefix(arr: &[&str]) -> String {
    if arr.len() == 0 {
        return String::new();
    }
    let mut result = String::new();
    let first: Vec<char> = arr[0].chars().collect();
    let rests: Vec<Vec<char>> = arr[1..].iter().map(|s| s.chars().collect()).collect();
    let mut i = 0;

    while i < first.len() {
        let ch = first[i];
        for j in 0..rests.len() {
            if i < rests[j].len() && ch == rests[j][i] {
                continue;
            } else {
                return result;
            }
        }
        result.push(ch);
        i += 1;
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
