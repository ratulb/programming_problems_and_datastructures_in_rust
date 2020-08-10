pub fn permutate(word: &String) -> Vec<String> {
    if word.len() <= 1 {
        return vec![word.to_string()];
    }

    let mut result: Vec<String> = Vec::with_capacity(factorial(word.len()));
    for i in 0..word.len() {
        let perms: Vec<String> = permutate(&(word[0..i].to_owned() + &word[i + 1..]));
        for mut perm in perms {
            perm.push(word.chars().nth(i).unwrap());
            result.push(perm);
        }
    }
    result
}

fn factorial(size: usize) -> usize {
    match size {
        0 | 1 => 1,
        _ => factorial(size - 1) * size,
    }
}

#[cfg(test)]
mod tests {
    use super::permutate;
    #[test]
    fn permutate_test_empty_char() {
        let word = permutate(&String::from(""));
        assert_eq!(vec![""], word);
    }

    #[test]
    fn permutate_test_one_char() {
        let word = permutate(&String::from("A"));
        assert_eq!(vec!["A"], word);
    }

    #[test]
    fn permutate_test() {
        let words = permutate(&String::from("ABC"));
        assert_eq!(vec!["CBA", "BCA", "CAB", "ACB", "BAC", "ABC"], words);
    }
}
