///Rotate a square matrix in place

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len() - 1;
    for i in 0..matrix.len() / 2 {
        for j in i..n - i {
            let top_left = matrix[i][j];
            matrix[i][j] = matrix[n - j][i];
            matrix[n - j][i] = matrix[n - i][n - j];
            matrix[n - i][n - j] = matrix[j][n - i];
            matrix[j][n - i] = top_left;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rotate;
    #[test]
    fn test_square_matrix_rotation() {
        let matrix = &mut vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(matrix);
        assert_eq!(*matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
