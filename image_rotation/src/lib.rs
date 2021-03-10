///Rotate a m by n matrix - can be m != n.
///Uses extra space

pub fn rotate_image(image: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = image.len();
    let cols = image[0].len();
    let mut copy = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            copy[i][j] = image[i][j];
        }
    }
    let mut transpose = transpose(&copy);
    //Reverse the rows of the transponsed matrix
    let rows = transpose.len();
    let cols = transpose[0].len();
    for i in 0..rows {
        let row = &mut transpose[i];
        let mut start = 0;
        let mut end = cols - 1;
        while start < end {
            let temp = row[start];
            row[start] = row[end];
            row[end] = temp;
            start += 1;
            end -= 1;
        }
    }
    transpose
}

///Transpose
///

pub fn transpose(image: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = image.len();
    let cols = image[0].len();
    let mut transpose = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transpose[j][i] = image[i][j];
        }
    }
    transpose
}

#[cfg(test)]
mod tests {
    use super::rotate_image;
    #[test]
    fn test_square_image_rotation() {
        let image = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            rotate_image(&image),
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
        );
    }
    #[test]
    fn test_rectangular_image_rotation() {
        let image = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            rotate_image(&image),
            vec![vec![4, 1], vec![5, 2], vec![6, 3]]
        );
    }
}
