pub fn count_bits(n: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(n + 1);
    result.push(0);
    let mut power = 1;
    for i in 1..=n {
        if i == power * 2 {
            power = i;
        }
        result.insert(i, 1 + result[i - power]);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits_1() {
        let result = count_bits(2);
        assert_eq!(result, vec![0, 1, 1]);
    }

    #[test]
    fn test_count_bits_2() {
        let result = count_bits(5);
        assert_eq!(result, vec![0, 1, 1, 2, 1, 2]);
    }
}
