///Maximum subsequence length of 3 strings
///Bottom up approach
use std::cmp::max;
pub fn max_sub_sequence_len(s1: &str, s2: &str, s3: &str) -> usize {
    if s1.len() == 0 || s2.len() == 0 || s3.len() == 0 {
        return 0;
    }
    let mut rect_cuboid = vec![vec![vec![0; s3.len() + 1]; s2.len() + 1]; s1.len() + 1];
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    let s3 = s3.chars().collect::<Vec<char>>();
    for i in 1..rect_cuboid.len() {
        for j in 1..rect_cuboid[i].len() {
            for k in 1..rect_cuboid[i][j].len() {
                if s1[i - 1] == s2[j - 1] && s2[j - 1] == s3[k - 1] {
                    rect_cuboid[i][j][k] = rect_cuboid[i - 1][j - 1][k - 1] + 1;
                } else {
                    rect_cuboid[i][j][k] = max(
                        max(rect_cuboid[i - 1][j][k], rect_cuboid[i][j - 1][k]),
                        rect_cuboid[i][j][k - 1],
                    );
                }
            }
        }
    }
    rect_cuboid[s1.len()][s2.len()][s3.len()]
}
#[cfg(test)]
mod tests {
    use super::max_sub_sequence_len;
    #[test]
    fn test_max_sub_sequence_len() {
        assert_eq!(max_sub_sequence_len("ABC1D", "ABC00D", "ABC2xyzD"), 4);
    }
}
