/***
 * Given an array of strings, find the logests common suffix for them
 ***/

pub fn longest_common_suffix(arr: &[&str]) -> String {
    if arr.len() == 0 {
        return String::new();
    }
    //Suffix would be as long as the min length of all the strings
    let mut result = String::with_capacity(
        arr.iter()
            .min_by(|p, n| p.len().cmp(&n.len()))
            .unwrap()
            .len(),
    );
    let mut first = arr[0].chars();
    let mut rests: Vec<_> = arr[1..].iter().map(|s| s.chars()).collect();
    while let Some(ch) = first.next_back() {
        for i in 0..rests.len() {
            let current = &mut rests[i];
            match current.next_back() {
                Some(c) if c == ch => continue,
                _ => return result,
            }
        }
        result.insert(0, ch);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_common_suffix_test() {
        let arr = ["privacy"];
        let result = longest_common_suffix(&arr);
        assert_eq!(result, String::from("privacy"));

        let arr = ["privacy", "fallacy", "delicacy"];
        let result = longest_common_suffix(&arr);
        assert_eq!(result, String::from("acy"));

        let arr = ["freedom", "kingdom", "boredom"];
        let result = longest_common_suffix(&arr);
        assert_eq!(result, String::from("dom"));
    }
}
