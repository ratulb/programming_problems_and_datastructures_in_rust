///Find in how many minimum jumps array end can be reached
///array_jumps contains max allowed jumps

pub fn reach_end_in_min_jumps(jumps: Vec<u32>) -> usize {
    if jumps.len() == 0 {
        return 0;
    }
    if jumps.len() == 1 {
        return 1;
    }
    if jumps[0] == 0 {
        return usize::MAX;
    }
    let mut solution = vec![0; jumps.len()];
    for j in 1..jumps.len() {
        let mut i = 0;
        while i < j {
            if j - i <= jumps[i] as usize {
                solution[j] = solution[i] + 1;
                break;
            }
            i += 1;
        }
        if i == j {
            return usize::MAX;
        }
    }
    solution[jumps.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::reach_end_in_min_jumps;
    #[test]
    fn test_reach_end_in_min_jumps_1() {
        assert_eq!(reach_end_in_min_jumps(vec![1, 0, 1]), usize::MAX);
    }
    #[test]
    fn test_reach_end_in_min_jumps_2() {
        assert_eq!(reach_end_in_min_jumps(vec![1, 3, 0, 0, 2, 0, 1]), 3);
    }
    #[test]
    fn test_reach_end_in_min_jumps_3() {
        assert_eq!(reach_end_in_min_jumps(vec![1, 1, 1, 1]), 3);
    }
    #[test]
    fn test_reach_end_in_min_jumps_4() {
        assert_eq!(reach_end_in_min_jumps(vec![1, 1, 4, 3, 0, 2, 0]), 3);
    }
}
