fn main() {
    println!("100 reversed {}", reverse(100));
    println!("123 reversed {}", reverse(123));
    println!("-123 reversed {}", reverse(-123));
    println!("1001 reversed {}", reverse(1001));
    println!("-100 reversed {}", reverse(-100));
    println!("-1001 reversed {}", reverse(-1001));
}

fn reverse(num: i32) -> i32 {
    if num.to_string().len() == 0 {
        return num;
    }
    let mut num = num;
    let mut is_negative = false;
    if num < 0 {
        is_negative = true;
        num = -num;
    }
    let mut result = 0;
    while num > 0 {
        result = result * 10 + num % 10;
        num = num / 10;
    }
    if is_negative {
        result * -1
    } else {
        result
    }
}
