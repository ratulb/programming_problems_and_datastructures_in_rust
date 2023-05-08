# Max vowels in substring

This code is used to find the maximum number of vowels in a given string within a given substring length. It takes in a string and an integer (k) as parameters, and returns the maximum number of vowels found in any substring of length k.

### Find maximum number of vowels
```rust,ignore
//Implementation

use std::collections::HashSet;
pub fn max_vowels(s: String, k: i32) -> i32 {
    let k: usize = k.try_into().unwrap();
    if s.len() < k.try_into().unwrap() {
        panic!(
            "IllegalArgumentException - string length {} is shorter than substring length {}",
            s.len(),
            k
        );
    }
    let s = s.chars().collect::<Vec<_>>();
    let vowels = "aeiou".chars().collect::<HashSet<_>>();
    let mut count = 0;
    let mut max_count = 0;
    for i in 0..s.len() {
        if vowels.contains(&s[i]) {
            count = count + 1;
        }
        if i < k {
            continue;
        }
        if vowels.contains(&s[i - k]) {
            count = count - 1;
        }
        max_count = std::cmp::max(count, max_count);
    }
    max_count
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/maximum-number-of-vowels/src/lib.rs).

