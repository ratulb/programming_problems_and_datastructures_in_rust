///Check if end of an array can be reached
///array_jumps contains max allowed jumps
pub fn reach_end_in_min_jumps(array_jumps: &[u32]) -> bool {
    if array_jumps.len() == 0 {
        return false;
    }
    let mut indices = Vec::<usize>::with_capacity(array_jumps.len());
    indices.push(0);
    while !indices.is_empty() {
        let index = indices.pop().unwrap();
        let steps = array_jumps[index];
        if index + steps as usize >= array_jumps.len() - 1 {
            return true;
        }
        for step in 1..=steps {
            let next_jump = index + step as usize;
            if next_jump <= array_jumps.len() - 2 && array_jumps[next_jump] != 0 {
                indices.push(next_jump);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::reach_end_in_min_jumps;
    #[test]
    fn test_reach_end_in_min_jumps_1() {
        assert_eq!(reach_end_in_min_jumps(&[1, 0, 1]), false);
    }
    #[test]
    fn test_reach_end_in_min_jumps_2() {
        assert_eq!(reach_end_in_min_jumps(&[1, 3, 0, 0, 2, 0, 1]), true);
    }
    #[test]
    fn test_reach_end_in_min_jumps_3() {
        assert_eq!(reach_end_in_min_jumps(&[1, 1, 1, 1]), true);
    }
    #[test]
    fn test_reach_end_in_min_jumps_4() {
        assert_eq!(reach_end_in_min_jumps(&[1, 1, 4, 3, 0, 2, 0]), true);
    }
}
