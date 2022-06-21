/***
 * Find max histogram area
 */
#[allow(unused_comparisons)]
pub fn max_area(heights: &[i32]) -> i32 {
    let mut max_area = 0i32;
    if heights.len() == 0 {
        return 0;
    } else if heights.len() == 1 {
        return heights[0];
    } else {
        for i in 0..heights.len() {
            let mut left = i;
            let mut right = i;

            while (left as i32 - 1) >= 0 && heights[left - 1] >= heights[i] {
                left -= 1;
            }
            while right + 1 < heights.len() && heights[right + 1] >= heights[i] {
                right += 1;
            }
            max_area = std::cmp::max(max_area, heights[i] * (right - left + 1) as i32);
        }
        max_area
    }
}

//Recursive way
use std::cmp::max;
pub fn max_area_recursive(heights: &[i32]) -> i32 {
    fn max_area_rec(heights: &[i32], low: i32, high: i32) -> i32 {
        println!("Low is {} and high is {}", low, high);
        if low > high {
            return 0;
        } else if low == high {
            return heights[low as usize];
        } else {
            let min_height = heights.iter().min();
            match min_height {
                None => return 0,
                Some(min_height) => {
                    let index_of_min = heights
                        .iter()
                        .enumerate()
                        .find(|pair| *pair.1 == *min_height);
                    match index_of_min {
                        None => return -1,
                        Some(idx_of_min) => {
                            /***let from_left = if idx_of_min.0 > 0 {
                                println!("Failing not here");
                                 let v = max_area_rec(heights, low, idx_of_min.0 - 1);
                                println!("Failing here");
                                 v
                            }else {
                                0
                            };***/
                            return {
                                max(
                                    *min_height * (high + 1 - low),
                                    max(
                                        max_area_rec(heights, low, idx_of_min.0 as i32 - 1),
                                        //from_left,
                                        max_area_rec(heights, idx_of_min.0 as i32 + 1, high),
                                    ),
                                )
                            };
                        }
                    }
                }
            }
        }
    }
    max_area_rec(heights, 0, (heights.len() - 1) as i32)
}

pub fn find_max(heights: &[u16]) -> u16 {
    if heights.len() == 0 {
        return 0;
    } else if heights.len() == 1 {
        return heights[0];
    } else {
        let mut area_max = 0;
        for i in 0..heights.len() {
            let mut min_height = heights[i];
            for j in i..heights.len() {
                min_height = std::cmp::min(min_height, heights[j]);
            }
            area_max = max(area_max, (heights.len() - i + 1) as u16 * min_height);
        }
        return area_max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let heights = [6, 2, 5, 4, 5, 1, 6];
        assert_eq!(max_area(&heights), 12);
        //assert_eq!(max_area_recursive(&heights), 12);
        let heights = [6, 2, 5, 4, 5, 1, 6];
        assert_eq!(find_max(&heights), 12);
    }
}
