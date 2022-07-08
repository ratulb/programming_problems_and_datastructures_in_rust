# Longest common prefix

### Find the longest common prefix for given array of strings:
```rust, ignore

/***
 * Find the longest prefix from an array of strings
 ***/

pub fn longest_common_prefix(arr: &[&str]) -> String {
    if arr.len() == 0 {
        return String::new();
    }
    //Prefix would be as long as the min length of all the strings
    let mut result = String::with_capacity(
        arr.iter()
            .min_by(|p, n| p.len().cmp(&n.len()))
            .unwrap()
            .len(),
    );
    //Get the characters of the first string in the in an iterator
    let mut first = arr[0].chars();
    //Map rests of the strings to iterator of chars and hold them in a vector
    let mut rests: Vec<_> = arr[1..].iter().map(|s| s.chars()).collect();
    //Iterate through the characters of the first string
    while let Some(ch) = first.next() {
        for i in 0..rests.len() {
            let current = &mut rests[i];
            match current.next() {
                //Next string's next char matches with the first string char
                //jump to the next string
                Some(c) if c == ch => continue,
                //Does not match - return whatever we have go so far
                _ => return result,
            }
        }
        //All strings next char matched, lengthen result
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
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/longest_common_prefix/src/lib.rs)
