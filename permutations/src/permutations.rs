pub fn permutate(s: String) {
  permutate_local(s,"".to_string());
}

fn permutate_local(s: String, acc: String) {
  if s.len() == 0 {
    println!("{}", acc);
  }
  let s_arr = &s[..];
  let mut index: usize = 0;
  for c in s.chars() {
    let mut acc = acc.clone();
    acc.push(c);
    let rest = s_arr[0..index].to_string() + &s_arr[index+1 ..];
    permutate_local(rest, acc);
    index = index+1;
  }
}
