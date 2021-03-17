///Remove dupcates from an array and return unique elements - input is assumed to be sorted

pub fn remove_duplicates(array: &mut [i32]) -> &[i32] {
    if array.len() == 0 {
        return array;
    }
    let mut index = 1;
    for i in 1..array.len() {
        if array[index - 1] != array[i] {
            array[index] = array[i];
            index += 1;
        }
    }
    &array[0..index]
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;
    #[test]
    fn test_remove_duplicates() {
        let array = &mut [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(array), &[0, 1, 2, 3, 4]);
    }
}
