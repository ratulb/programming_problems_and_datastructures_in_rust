# String permutation

### Given string s, find all the permutations of its characters:
```rust,ignore
/***
 * Given a string, find all the unique permutations of its characters
 ***/
pub fn permutate(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    if s.len() == 0 {
        return result;
    }
    if s.len() == 1 {
        result.push(s.to_string());
        return result;
    }
    let chars: Vec<_> = s.chars().collect();
    for i in 0..chars.len() {
        let ch = chars[i];
        let mut segment = String::from(&s[0..i]);
        segment += &s[i + 1..];
        let intermediates = permutate(&segment);
        for mut s in intermediates {
            s.push(ch);
            result.push(s);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::permutate;
    #[test]
    fn permutate_test() {
        let result = permutate("abc");
        assert_eq!(
            result,
            vec![
                String::from("cba"),
                String::from("bca"),
                String::from("cab"),
                String::from("acb"),
                String::from("bac"),
                String::from("abc")
            ]
        );
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/tree/master/permutations/src/lib.rs)
