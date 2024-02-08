/***
 Given an arithmetic expression in Reverse Polish Notation, write a program to evaluate it.

The expression is given as a list of numbers and operands. For example: [5, 3, '+'] should return 5 + 3 = 8.

For example, [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'] should return 5, since it is equivalent to ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) = 5.

You can assume the given expression is always valid.

***/

pub fn evaluate(s: &str) -> Result<i32, &'static str> {
    let mut splits: Vec<&str> = s.split(",").map(|s| s.trim()).collect();
    let mut stack = Vec::with_capacity(splits.len());
    while !splits.is_empty() {
        let curr = splits.remove(0);
        match curr.parse::<i32>() {
            Ok(num) => stack.push(num),
            Err(_) => match curr.chars().nth(1) {
                Some(ch) if stack.len() >= 2 => match run_op(ch, &mut stack) {
                    Ok(()) => continue,
                    Err(msg) => return Err(msg),
                },
                Some(_) if stack.len() < 2 => return Err("Not enough operands"),
                _ => return Err("Unexpected input encountered!"),
            },
        }
    }
    if !splits.is_empty() || stack.len() != 1 {
        return Err("Unexpected input");
    }
    Ok(stack.remove(0))
}

fn run_op(ch: char, stack: &mut Vec<i32>) -> Result<(), &'static str> {
    let len = stack.len() - 1;
    let top = stack.remove(len);
    let top_next = stack.remove(len - 1);

    match ch {
        '+' => stack.push(top + top_next),
        '-' => stack.push(top_next - top),
        '*' => stack.push(top * top_next),
        '/' if top != 0 => stack.push(top_next / top),
        _ => return Err("Invalid operator"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_test_1() {
        let s = "15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'";
        let result = evaluate(s);
        assert_eq!(result, Ok(5));

        let s = "15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-', '*'";
        let result = evaluate(s);
        assert_eq!(result, Err("Not enough operands"));
    }

    #[test]
    fn evaluate_test_2() {
        let s = "5, 3, '+'";
        let result = evaluate(s);
        assert_eq!(result, Ok(8));

        let s = "5, 3, '+', 100, '-'";
        let result = evaluate(s);
        assert_eq!(result, Ok(-92));
    }
}
