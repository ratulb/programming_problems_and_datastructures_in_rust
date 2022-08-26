# Pascal's triangle

### Generate rows of Pascal's triangle
```rust, ignore
pub fn generate(rows: usize) -> Vec<Vec<usize>> {
    if rows == 0 {
        return vec![];
    }
    let mut triangle = Vec::with_capacity(rows);
    triangle.push(vec![1]);
    for i in 1..rows {
        let prev_row = &triangle[i - 1];
        let mut curr_row: Vec<_> = vec![1];
        for j in 1..i {
            curr_row.push(prev_row[j - 1] + prev_row[j]);
        }
        curr_row.push(1);
        triangle.push(curr_row);
    }
    triangle
}
```
### Find the nth row of pascal's triangle recursivley
```rust, ignore
pub fn nth(row: usize) -> Vec<usize> {
    match row {
        n if n < 1 => vec![],
        n if n == 1 => vec![1],
        n => {
            let prev_row = nth(n - 1);
            let mut curr = Vec::with_capacity(n);
            curr.push(1);
            for i in 1..prev_row.len() {
                curr.push(prev_row[i - 1] + prev_row[i]);
            }
            curr.push(1);
            curr
        }
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/pascal_triangle/src/lib.rs)
