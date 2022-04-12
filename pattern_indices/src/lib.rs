/***
 * Given a string and a pattern, find the starting indices of all occurrences of the pattern in the string. For example, given the string "abracadabra" and the pattern "abr", you should return [0, 7]
 *
 ***/

pub fn pattern_indices(s: String, pat: &str) -> Vec<usize> {
    if s.len() == 0 || pat.len() == 0 || s.len() < pat.len() {
        return vec![];
    }
    let mut s = s;
    let mut result = vec![];
    let mut offset = 0;
    while let Some(index) = s.find(pat) {
        let (_, rest) = s.split_at(index + pat.len());
        result.push(offset + index);
        offset += index + pat.len();
        s = rest.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let result = pattern_indices("abracadabra".to_string(), "abr");
        assert_eq!(result, vec![0, 7]);
    }
    #[test]
    fn test2() {
        let result = pattern_indices("abracadabraabrabr".to_string(), "abr");
        assert_eq!(result, vec![0, 7, 11, 14]);
    }
    #[test]
    fn test3() {
        let result = pattern_indices("ABCDMASSDDDDDMASSHHHHSAMASSGGH".to_string(), "MASS");
        assert_eq!(result, vec![4, 13, 23]);
    }

    #[test]
    fn test4() {
        let s = String::from("ABABABABABABABABABAB");
        let result = pattern_indices("ABABABABABABABABABAB".to_string(), "A");
        assert_eq!(result, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }
}
