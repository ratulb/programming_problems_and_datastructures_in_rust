#![allow(unused_comparisons)]
/***
 * Search a key in a matrix where row as well as colum values are sorted in ascending order
 ***/
pub fn search(matrix: &[&[i32]], key: i32) -> Option<(usize, usize)> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut row: usize = 0;
    let mut col: usize = cols - 1;
    while (row >= 0 && row < rows) && (col >= 0 && col < cols) {
        if matrix[row][col] == key {
            return Some((row, col));
        } else if matrix[row][col] > key {
            if col > 0 {
                col -= 1;
            } else {
                return None;
            }
        } else if matrix[row][col] < key {
            row += 1;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::search;
    #[test]
    fn test_search() {
        let row1 = [12, 34, 45, 67];
        let row2 = [14, 35, 47, 78];
        let row3 = [19, 37, 51, 81];
        let matrix = [&row1[..], &row2[..], &row3[..]];
        assert_eq!(search(&matrix[..], 51), Some((2, 2)));
        assert_eq!(search(&matrix[..], 510), None);
    }
}
