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
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Operand(num) => write!(f, "{}", num),
            Token::Operator(ch) => write!(f, "{}", ch),
        }
    }
}

impl Token {
    pub fn is_negation(&self) -> bool {
        match self {
            Token::Operand(_) => false,
            Token::Operator(ch) => *ch == '-',
        }
    }
    pub fn negate(&mut self) {
        if let Token::Operand(ref mut num) = self {
            *num *= -1;
        }
    }
    pub fn invert(&mut self) {
        match self {
            Token::Operator(ref mut op) if *op == '-' => *op = '+',
            _ => (),
        }
    }
}

// Function to tokenize the expression
pub fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buffer = String::new();

    for c in expression.chars() {
        match c {
            '+' | '-' | '*' | '/' | '%' | '(' | ')' => {
                if !num_buffer.is_empty() {
                    tokens.push(Token::Operand(num_buffer.parse().unwrap()));
                    num_buffer.clear();
                }
                tokens.push(Token::Operator(c));
            }
            '0'..='9' => {
                num_buffer.push(c);
            }
            _ => {} // Ignore whitespace and other characters
        }
    }

    // Push the last operand if there's anything left in the buffer
    if !num_buffer.is_empty() {
        tokens.push(Token::Operand(num_buffer.parse().unwrap()));
    }

    tokens
}

fn main() {
    let exp = "((3 + 4) * (-3) + 990 % 30 + (100 + 10))";
    let tokens = tokenize(exp);
    println!("Tokens: {:?}", tokens);
    //(, (, 3, +, 4, ), *, (, -, 3, ), +, 990, %, 30, +, (, 100, +, 10, ), )
}
