///Find next greater number with same set of digits

pub fn next_higher_num(num: u32) -> Option<u32> {
    let mut chars: Vec<char> = num.to_string().chars().collect();
    let len = chars.len();
    if descending(&chars) {
        None
    } else if ascending(&chars) {
        swap(&mut chars, len - 2, len - 1);
        chars
            .into_iter()
            .map(|c| c.to_digit(10))
            .try_fold(0, |ans, i| i.map(|i| ans * 10 + i))
    } else {
        let last = chars[len - 1];
        let mut i = len - 2;
        while i >= 0 {
            if last > chars[i] {
                break;
            }
            i -= 1;
        }
        swap(&mut chars, i, len - 1);
        let mut vec_split = chars.split_off(i + 1);
        vec_split.sort();
        chars.append(&mut vec_split);
        chars
            .into_iter()
            .map(|c| c.to_digit(10))
            .try_fold(0, |ans, i| i.map(|i| ans * 10 + i))
    }
}
fn swap(chars: &mut Vec<char>, i: usize, j: usize) {
    let ch = chars[i];
    chars[i] = chars[j];
    chars[j] = ch;
}
fn ascending(chars: &Vec<char>) -> bool {
    let mut ascending = true;
    for i in 1..chars.len() {
        if chars[i - 1] > chars[i] {
            ascending = false;
            break;
        }
    }
    ascending
}
fn descending(chars: &Vec<char>) -> bool {
    let mut descending = true;
    for i in 1..chars.len() {
        if chars[i - 1] < chars[i] {
            descending = false;
        }
    }
    descending
}

#[cfg(test)]
mod tests {
    use super::next_higher_num;
    #[test]
    fn test_next_higher_num1() {
        assert_eq!(next_higher_num(1), None);
    }
    #[test]
    fn test_next_higher_num2() {
        assert_eq!(next_higher_num(123), Some(132));
    }
    #[test]
    fn test_next_higher_num3() {
        assert_eq!(next_higher_num(3210), None);
    }
    #[test]
    fn test_next_higher_num4() {
        assert_eq!(next_higher_num(534976), Some(536479));
    }
}
