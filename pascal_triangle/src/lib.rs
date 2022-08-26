/***
 * Generate Pascal's triangle
 */

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_row() {
        assert_eq!(generate(0), Vec::<Vec<_>>::new());
    }
    #[test]
    fn test_one_row() {
        let row1 = vec![1];
        let triangle = vec![row1];
        assert_eq!(generate(1), triangle);
    }
    #[test]
    fn test_multi_rows() {
        let row1 = vec![1];
        let row2 = vec![1, 1];
        let row3 = vec![1, 2, 1];
        let row4 = vec![1, 3, 3, 1];
        let row5 = vec![1, 4, 6, 4, 1];
        let triangle = vec![row1, row2, row3, row4, row5];
        assert_eq!(generate(5), triangle);
    }

    #[test]
    fn test_nths() {
        assert_eq!(nth(0), vec![]);
        assert_eq!(nth(1), vec![1]);
        assert_eq!(nth(2), vec![1, 1]);
        assert_eq!(nth(3), vec![1, 2, 1]);
        assert_eq!(nth(4), vec![1, 3, 3, 1]);
        assert_eq!(nth(5), vec![1, 4, 6, 4, 1]);
    }
}
