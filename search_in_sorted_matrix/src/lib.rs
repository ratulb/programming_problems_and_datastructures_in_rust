pub fn search(matrix: &[&[i32]], key: i32) -> (i32, i32) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut i: usize = 0;
    let not_found = (-1, -1);
    let mut j: usize = cols - 1;
    while (i >= 0 && i < rows) && (j >= 0 && j < cols) {
        if matrix[i][j] == key {
            return (i as i32, j as i32);
        } else if matrix[i][j] > key {
            if j > 0 {
                j -= 1;
            } else {
                return not_found;
            }
        } else if matrix[i][j] < key {
            i += 1;
        }
    }
    return not_found;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        let row1 = [12, 34, 45, 67];
        let row2 = [14, 35, 47, 78];
        let row3 = [19, 37, 51, 81];
        let matrix = [&row1[..], &row2[..], &row3[..]];
        assert_eq!(search(&matrix[..], 51), (2, 2));
        assert_eq!(search(&matrix[..], 510), (-1, -1));
    }
}
