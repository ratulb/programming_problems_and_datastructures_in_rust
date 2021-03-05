#[cfg(test)]
pub fn first_non_repeating(s: &String) -> Option<char> {
  let mut counts = [0;26];
  for ch in s.chars() {
    let index = (ch as u32 - 'a' as u32) as usize;
    counts[index] = counts[index] + 1;
  }
  for ch in s.chars() {
     let index =  (ch as u32 - 'a' as u32) as usize;
     if counts[index] == 1 {
       return Some(ch);
     }
  }
  println!("The counts array {:?}", counts);
  None
}

mod tests {
    use super::*;
    #[test]
    fn empty_string_test() {
        assert_eq!(first_non_repeating(&String::new()), None);
    }
    #[test]
    fn unique_string_test() {
        assert_eq!(first_non_repeating(&String::from("abc")), Some('a'));
    }
    #[test]
    fn non_unique_string_test1() {
        assert_eq!(first_non_repeating(&String::from("abcabcde")), Some('d'));
    }
    #[test]
    fn non_unique_string_test2() {
        assert_eq!(first_non_repeating(&String::from("abcabfcd")), Some('f'));
    }
}
