///Find the total rain water trapped among series of buildings given their heights

pub fn water_trapped(heights: &[u32]) -> u32 {
    if heights.len() == 0 {
        return 0;
    }
    if heights.iter().all(|&h| h == heights[0]) {
        return 0;
    }
    let mut max_heights_left: Vec<u32> = vec![0; heights.len()];
    let mut max_heights_right: Vec<u32> = vec![0; heights.len()];
    max_heights_left[0] = heights[0];
    max_heights_right[heights.len() - 1] = heights[heights.len() - 1];

    for i in 1..heights.len() {
        max_heights_left[i] = std::cmp::max(max_heights_left[i - 1], heights[i]);
    }
    for j in (0..heights.len() - 1).rev() {
        max_heights_right[j] = std::cmp::max(max_heights_right[j + 1], heights[j]);
    }
    let mut water = 0;
    for i in 0..heights.len() {
        water += std::cmp::min(max_heights_left[i], max_heights_right[i]) - heights[i];
    }
    water
}

#[cfg(test)]
mod tests {
    use super::water_trapped;
    #[test]
    fn test_all_equal_heights() {
        assert_eq!(water_trapped(&[5, 5, 5]), 0);
    }
    #[test]
    fn test_unequal_heights_1() {
        assert_eq!(water_trapped(&[5, 5, 5, 7]), 0);
    }
    #[test]
    fn test_unequal_heights_2() {
        assert_eq!(water_trapped(&[5, 7, 5, 7]), 2);
    }
    #[test]
    fn test_unequal_heights_3() {
        assert_eq!(water_trapped(&[1, 2, 1, 2]), 1);
    }
    #[test]
    fn test_unequal_heights_4() {
        assert_eq!(water_trapped(&[0, 2, 4, 0, 2, 1, 2, 6]), 11);
    }
}
