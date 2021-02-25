use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?} ", permutate_iterative(args[1].to_owned()));
}

pub fn permutate(s: String) -> Vec<String> {
    let mut result = vec![];
    if s.len() == 0 {
        result
    } else if s.len() == 1 {
        result.push(s);
        result
    } else {
        let mut index = 0;
        for letter in s.chars() {
            let intermediates = permutate(String::from(&s[0..index]) + &s[index + 1..]);
            for perm in intermediates {
                result.push(perm + &letter.to_string());
            }
            index = index + 1;
        }
        result
    }
}

pub fn permutate_iterative(s: String) -> Vec<String> {
    let mut result = vec![];
    if s.len() == 0 {
        result
    } else if s.len() == 1 {
        result.push(s);
        result
    } else {
        let mut letters = vec![];
        for c in s.chars() {
          letters.push(c);
        }
        result.push(letters.pop().unwrap().to_string());
        while !letters.is_empty() {
           let letter = letters.pop().unwrap();
           let mut intermediates = vec![];
           for perm in result {
             for i in 0..=perm.len(){
               let mut temp_string = String::from(&perm[0..i]) + &letter.to_string();
               temp_string = temp_string + &perm[i..];
               intermediates.push(temp_string);
             }
           }
           result = intermediates;
        }
        result
    }
}
