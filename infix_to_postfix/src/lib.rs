//Given an infix expression, convert it to postfix

use Token::{NotSupported, Operand, Operator};

fn append_to_result(token: Token, result: &mut Vec<String>) {
    if !result.is_empty() {
        result.push(", ".to_string());
    }
    match token {
        num @ Operand(_) => result.push(num.into()),
        Operator(ch) => {
            let mut s = String::from("'");
            s += &ch.to_string();
            s += "'";
            result.push(s);
        }
        NotSupported(_) => (),
    }
}

pub fn postfix(infix_expr: &str) -> Result<String, &str> {
    if infix_expr.len() == 0 {
        return Err("Empty infix expression");
    }
    let tokens = tokenize(infix_expr);
    if tokens.len() < 2 {
        return Err("Not enough tokens after tokenizing");
    }

    let mut result = Vec::with_capacity(tokens.len());
    let mut stack = Vec::with_capacity(tokens.len());
    for token in tokens {
        match token {
            num @ Operand(_) => append_to_result(num, &mut result),
            Operator(')') if stack.is_empty() => return Err("Unmatched infix expression[')'] "),
            Operator(ch) if stack.is_empty() => stack.push(ch),
            Operator(')') => {
                while !stack.is_empty() && stack[stack.len() - 1] != '(' {
                    append_to_result(Operator(stack.remove(stack.len() - 1)), &mut result);
                }
                _ = stack.pop();
            }
            Operator(ch)
                if (ch == '*' || ch == '/')
                    && (stack[stack.len() - 1] == '+' || stack[stack.len() - 1] == '-') =>
            {
                stack.push(ch)
            }

            Operator(ch) if ch == '+' || ch == '-' => {
                while !stack.is_empty()
                    && (stack[stack.len() - 1] == '+' || stack[stack.len() - 1] == '-')
                {
                    append_to_result(Operator(stack.remove(stack.len() - 1)), &mut result);
                }
                stack.push(ch);
            }
            Operator(ch) if ch == '*' || ch == '/' => {
                while !stack.is_empty()
                    && (stack[stack.len() - 1] == '*' || stack[stack.len() - 1] == '/')
                {
                    append_to_result(Operator(stack.remove(stack.len() - 1)), &mut result);
                }
                stack.push(ch);
            }

            Operator(ch) => {
                stack.push(ch);
            }

            NotSupported(ch) => {
                eprintln!("Unsupported token in expression {}", ch);
                println!("Unsupported token in expression {}", ch);
                return Err("Unsupported token in expression");
            }
        }
    }

    while let Some(ch) = stack.pop() {
        append_to_result(ch.into(), &mut result);
    }

    let mut s = String::with_capacity(result.len());
    for token in result {
        s.push_str(&token);
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postfix_test_1() {
        let result = postfix("");
        assert!(result == Err("Empty infix expression"));
        let result = postfix("2 + 1");
        assert_eq!(result, Ok("2, 1, '+'".to_string()));

        let result = postfix("2 +   1 -  100");
        //assert_eq!(result, Ok("2, 1, 100, '-', '+'".to_string()));
        assert_eq!(result, Ok("2, 1, '+', 100, '-'".to_string()));

        let infix = "( ( 15 / ( 7 - ( 1 + 1 ) ) ) * 3 ) - ( 2 + ( 1 + 1 ) )";
        let result = postfix(infix);
        assert_eq!(
            result,
            Ok("15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'".to_string())
        );

        let infix = "5 + 3";
        let result = postfix(infix);
        assert_eq!(result, Ok("5, 3, '+'".to_string()));
    }
}
/***To generate a token tree of operands and operators in Rust from the given expression, you can tokenize the expression first and then build the token tree. Here's a step-by-step approach to accomplish this:

Tokenization: Break down the expression into individual tokens. Tokens can be operands (numbers) or operators (+, -, *, /, %, etc.). You'll also need to consider parentheses as tokens.

Token Tree Construction: Once you have tokens, you can build a token tree using Rust's data structures. One approach is to use an enum to represent tokens, where each variant holds either an operand or an operator. You can then construct a binary tree where the operands are the leaves and the operators are the internal nodes.

Here's a Rust code example to illustrate these steps:

***/
use std::fmt;
// Define the Token enum to represent operands and operators
pub enum Token {
    Operand(i32),
    Operator(char),
    NotSupported(char),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Operand(num) => write!(f, "{}", num),
            Token::Operator(ch) => write!(f, "{}", ch),
            Token::NotSupported(ch) => write!(f, "{}", ch),
        }
    }
}

impl From<Token> for String {
    fn from(token: Token) -> String {
        match token {
            Token::Operand(num) => num.to_string(),
            Token::Operator(ch) => ch.to_string(),
            Token::NotSupported(ch) => ch.to_string(),
        }
    }
}

impl From<char> for Token {
    fn from(ch: char) -> Token {
        match ch {
            '+' | '-' | '*' | '/' | '%' | '(' | ')' => Token::Operator(ch),
            '0'..='9' => Token::Operand(ch.to_string().parse::<i32>().unwrap()),
            _ => Token::NotSupported(ch),
        }
    }
}

// Function to tokenize the expression
pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buffer = String::new();

    for ch in expression.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '%' | '(' | ')' => {
                if !num_buffer.is_empty() {
                    tokens.push(Token::Operand(num_buffer.parse().unwrap()));
                    num_buffer.clear();
                }
                tokens.push(Token::Operator(ch));
            }
            '0'..='9' => {
                num_buffer.push(ch);
            }
            _ => {
                //eprintln!("Ignoring not supported character {}", ch);
            } // Ignore whitespace and other characters
        }
    }

    // Push the last operand if there's anything left in the buffer
    if !num_buffer.is_empty() {
        tokens.push(Token::Operand(num_buffer.parse().unwrap()));
    }
    tokens
}

