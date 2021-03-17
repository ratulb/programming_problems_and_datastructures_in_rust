///Print a matrix spirally

pub fn spiral_print(matrix: &Vec<Vec<i32>>) {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        return;
    }
    let mut left = 0;
    let mut down = 0;
    let mut right = matrix[0].len() - 1;
    let mut up = matrix.len() - 1;
    let mut dir = 0;

    while left <= right && down <= up {
        if dir == 0 {
            for j in left..=right {
                print!("{} ", matrix[down][j]);
            }
            down += 1;
            dir = 1;
        } else if dir == 1 {
            for i in down..=up {
                print!("{} ", matrix[i][right]);
            }
            right -= 1;
            dir = 2;
        } else if dir == 2 {
            for j in (left..=right).rev() {
                print!("{} ", matrix[up][j]);
            }
            up -= 1;
            dir = 3;
        } else if dir == 3 {
            for i in (up..=down).rev() {
                print!("{} ", matrix[i][left]);
            }
            left += 1;
            dir = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::spiral_print;
    #[test]
    fn test_spiral_print() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        spiral_print(&matrix);
    }
}
