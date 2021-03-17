///Remove duplicates from a vec

pub fn remove_duplicates(vec: &mut Vec<i32>) {
    if vec.len() == 0 {
        return;
    }
    let mut index = 1;
    for i in 1..vec.len() {
        if vec[index - 1] != vec[i] {
            vec[index] = vec[i];
            index += 1;
        }
    }
    let mut end = vec.len() - 1;
    while end >= index {
        vec.remove(end);
        end = vec.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;
    #[test]
    fn test_remove_dupliactes() {
        let vec = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        remove_duplicates(vec);
        assert_eq!(vec, &mut vec![0, 1, 2, 3, 4]);
    }
}
