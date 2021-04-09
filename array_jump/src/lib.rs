///Check if end of an array can be reached - if so - in how many minimum jumps
///array_jumps contains max allowed jumps
pub fn reach_end_in_min_jumps(array_jumps: &[u32]) -> (bool, usize) {
    let mut reachable_in_jumps = (false, usize::MAX);
    if array_jumps.len() == 0 {
        return reachable_in_jumps;
    }
    let mut min_jumps = 0;
    let mut indices = Vec::<usize>::with_capacity(array_jumps.len());
    indices.push(0);
    while !indices.is_empty() {
        let index = indices.pop().unwrap();
        min_jumps += 1;
        let steps = array_jumps[index];
        if index + steps as usize >= array_jumps.len() - 1 {
            reachable_in_jumps.0 = true;
            reachable_in_jumps.1 = min_jumps;
            return reachable_in_jumps;
        }
        for step in 1..=steps {
            let next_jump = index + step as usize;
            if next_jump <= array_jumps.len() - 2 && array_jumps[next_jump] != 0 {
                indices.push(next_jump);
            }
        }
    }
    reachable_in_jumps
}

#[cfg(test)]
mod tests {
    use super::reach_end_in_min_jumps;
    #[test]
    fn test_reach_end_in_min_jumps_1() {
        assert_eq!(reach_end_in_min_jumps(&[1, 0, 1]), (false, usize::MAX));
    }
    #[test]
    fn test_reach_end_in_min_jumps_2() {
        assert_eq!(reach_end_in_min_jumps(&[1, 3, 0, 0, 2, 0, 1]), (true, 3));
    }
    #[test]
    fn test_reach_end_in_min_jumps_3() {
        assert_eq!(reach_end_in_min_jumps(&[1, 1, 1, 1]), (true, 3));
    }
    #[test]
    fn test_reach_end_in_min_jumps_4() {
        assert_eq!(reach_end_in_min_jumps(&[1, 1, 4, 3, 0, 2, 0]), (true, 3));
    }
}
