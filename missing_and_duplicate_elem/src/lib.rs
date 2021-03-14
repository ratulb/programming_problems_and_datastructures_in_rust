///Find th missing and in an array containing elements 1 to N
use std::collections::HashMap;
pub fn missing_and_duplicate(array: &[usize]) -> Option<(usize, usize)> {
    if array.len() == 0 {
        return None;
    } else {
        let mut elem_counter = HashMap::<usize, usize>::new();
        for elem in array {
            let entry = elem_counter.entry(*elem).or_insert(0);
            *entry += 1;
        }
        let mut result = (0, 0);
        for i in 1..array.len() {
            match elem_counter.get(&i) {
                Some(v) if *v > 1 => result.1 = i,
                None => result.0 = i,
                Some(_) => (),
            };
        }
        match result {
            (0, 0) => return None,
            _ => return Some(result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::missing_and_duplicate;
    #[test]
    fn test_missing_and_duplicate() {
        assert_eq!(missing_and_duplicate(&[1, 2, 4, 1, 5]), Some((3, 1)));
    }
}
