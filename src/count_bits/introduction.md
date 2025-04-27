# Count 1s For Each Entry
 Return an array where each element at index i (0 ≤ i ≤ n) is the number of 1's in the binary representation of i.


```rust,ignore
pub fn count_bits(n: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(n + 1);
    result.push(0);
    let mut power = 1;
    for i in 1..=n {
        if i == power * 2 {
            power = i;
        }
        result.insert(i, 1 + result[i - power]);
    }
    return result;
}

```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/count_bits/src/lib.rs)
