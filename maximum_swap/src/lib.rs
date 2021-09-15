///670. Maximum Swap - Leetcode

pub fn maximum_swap(num: i32) -> i32 {
    let num = num.to_string();
    let digits: Vec<_> = num
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect();
    let mut index = 0;
    let mut max_index;
    let mut possible_swaps = vec![];
    loop {
        max_index = index;
        let mut max_val = digits[index];
        for i in index..digits.len() {
            if digits[i] >= max_val {
                max_val = digits[i];
                max_index = i;
            }
        }
        if index != max_index {
            possible_swaps.push((index, max_index));
        }
        index += 1;
        if index >= digits.len() {
            break;
        }
    }
    let mut max_num = digits.iter().fold(0, |acc, d| acc * 10 + d);
    possible_swaps.iter().for_each(|idx_max_pair| {
        let curr_sum = digits.iter().enumerate().fold(0, |acc, tuple| {
            let idx = tuple.0;
            let value = tuple.1;
            let index = idx_max_pair.0;
            let max_index = idx_max_pair.1;
            if idx == index {
                acc * 10 + digits[max_index]
            } else if idx == max_index {
                acc * 10 + digits[index]
            } else {
                acc * 10 + value
            }
        });
        max_num = std::cmp::max(max_num, curr_sum);
    });
    max_num
}

#[cfg(test)]
mod tests {
    use super::maximum_swap;
    #[test]
    fn maximum_swap_test() {
       assert_eq!(maximum_swap(1993), 9913);
       assert_eq!(maximum_swap(990199), 999190);
    }
}
