use std::io;
fn main() {
    println!("Enter a striing to reverse");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read");
    println!("{}", reverse(s));
}

pub fn reverse(s: String) -> String {
    type_of(&s);
    let mut reversed = String::new();
    for c in s.chars() {
      reversed = String::from(&c.to_string()) + &reversed;
    }
    reversed
}

fn type_of<T>(_: T) {
  println!("The type is {}", std::any::type_name::<T>());
}
