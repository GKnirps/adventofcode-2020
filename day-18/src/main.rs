use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let tokenized = tokenize_input(&content)?;

    let sum = tokenized
        .iter()
        .map(|tokens| run_expression(tokens))
        .sum::<Result<u64, String>>()?;
    println!("Sum of all expressions is: {}", sum);

    let advanced_sum = tokenized
        .iter()
        .map(|tokens| run_expression_advanced(tokens))
        .sum::<Result<u64, String>>()?;
    println!("Sum with advanced math is: {}", advanced_sum);

    Ok(())
}

fn run_expression(tokens: &[Token]) -> Result<u64, String> {
    let mut output_queue: Vec<Token> = Vec::with_capacity(128);
    let mut op_stack: Vec<Token> = Vec::with_capacity(128);
    for token in tokens {
        match token {
            Token::Num(_) => output_queue.push(*token),
            Token::Mul | Token::Add => {
                while let Some(op) = op_stack.last().copied() {
                    if op == Token::Mul || op == Token::Add {
                        op_stack.pop();
                        output_queue.push(op);
                    } else {
                        break;
                    }
                }
                op_stack.push(*token);
            }
            Token::ParO => op_stack.push(*token),
            Token::ParC => {
                while let Some(op) = op_stack.last() {
                    if *op == Token::ParO {
                        break;
                    }
                    output_queue.push(*op);
                    op_stack.pop();
                }
                if op_stack.pop() != Some(Token::ParO) {
                    return Err("Mismatching paranthesis".to_owned());
                }
            }
        }
    }
    while let Some(op) = op_stack.pop() {
        output_queue.push(op);
    }

    let mut num_stack: Vec<u64> = Vec::with_capacity(output_queue.len());
    for token in output_queue {
        match token {
            Token::Num(n) => num_stack.push(n),
            Token::Mul => {
                // the algorithm before should have assured we have a valid expression on the stack
                let n1 = num_stack.pop().expect("not enough operands on stack");
                let n2 = num_stack.pop().expect("not enough operands on stack");
                num_stack.push(n1 * n2);
            }
            Token::Add => {
                // the algorithm before should have assured we have a valid expression on the stack
                let n1 = num_stack.pop().expect("not enough operands on stack");
                let n2 = num_stack.pop().expect("not enough operands on stack");
                num_stack.push(n1 + n2);
            }
            _ => return Err(format!("Unknown operator on stack: {:?}", token)),
        }
    }
    num_stack
        .first()
        .copied()
        .ok_or_else(|| "No result after executing expression".to_owned())
}

fn run_expression_advanced(tokens: &[Token]) -> Result<u64, String> {
    let mut output_queue: Vec<Token> = Vec::with_capacity(128);
    let mut op_stack: Vec<Token> = Vec::with_capacity(128);
    for token in tokens {
        match token {
            Token::Num(_) => output_queue.push(*token),
            Token::Mul => {
                while let Some(op) = op_stack.last().copied() {
                    if op == Token::Mul || op == Token::Add {
                        op_stack.pop();
                        output_queue.push(op);
                    } else {
                        break;
                    }
                }
                op_stack.push(*token);
            }
            Token::Add => {
                while let Some(op) = op_stack.last().copied() {
                    if op == Token::Add {
                        op_stack.pop();
                        output_queue.push(op);
                    } else {
                        break;
                    }
                }
                op_stack.push(*token);
            }
            Token::ParO => op_stack.push(*token),
            Token::ParC => {
                while let Some(op) = op_stack.last() {
                    if *op == Token::ParO {
                        break;
                    }
                    output_queue.push(*op);
                    op_stack.pop();
                }
                if op_stack.pop() != Some(Token::ParO) {
                    return Err("Mismatching paranthesis".to_owned());
                }
            }
        }
    }
    while let Some(op) = op_stack.pop() {
        output_queue.push(op);
    }

    let mut num_stack: Vec<u64> = Vec::with_capacity(output_queue.len());
    for token in output_queue {
        match token {
            Token::Num(n) => num_stack.push(n),
            Token::Mul => {
                // the algorithm before should have assured we have a valid expression on the stack
                let n1 = num_stack.pop().expect("not enough operands on stack");
                let n2 = num_stack.pop().expect("not enough operands on stack");
                num_stack.push(n1 * n2);
            }
            Token::Add => {
                // the algorithm before should have assured we have a valid expression on the stack
                let n1 = num_stack.pop().expect("not enough operands on stack");
                let n2 = num_stack.pop().expect("not enough operands on stack");
                num_stack.push(n1 + n2);
            }
            _ => return Err(format!("Unknown operator on stack: {:?}", token)),
        }
    }
    num_stack
        .first()
        .copied()
        .ok_or_else(|| "No result after executing expression".to_owned())
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Token {
    Num(u64),
    Mul,
    Add,
    ParO,
    ParC,
}

fn tokenize_input(input: &str) -> Result<Vec<Vec<Token>>, String> {
    input.split_terminator('\n').map(tokenize_line).collect()
}

fn tokenize_line(line: &str) -> Result<Vec<Token>, String> {
    // the input looks like that all numbers are single-digit numbers, so I won't bother parsing
    // longer numbers
    line.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            c.to_digit(10)
                .map(|n| Token::Num(n as u64))
                .or_else(|| match c {
                    '*' => Some(Token::Mul),
                    '+' => Some(Token::Add),
                    '(' => Some(Token::ParO),
                    ')' => Some(Token::ParC),
                    _ => None,
                })
                .ok_or_else(|| format!("Unknown character in expression: '{}'", c))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_expression_works_for_examples() {
        // given
        let examples = r"2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let tokenized = tokenize_input(examples).expect("Expected successful tokenization");

        // when
        let results = tokenized
            .iter()
            .map(|tokens| run_expression(tokens))
            .collect::<Result<Vec<u64>, String>>()
            .expect("Expected successful parsing");

        // then
        assert_eq!(results, &[26, 437, 12240, 13632]);
    }

    #[test]
    fn run_expression_advanced_works_for_examples() {
        // given
        let examples = r"1 + (2 * 3) + (4 * (5 + 6))
        2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let tokenized = tokenize_input(examples).expect("Expected successful tokenization");

        // when
        let results = tokenized
            .iter()
            .map(|tokens| run_expression_advanced(tokens))
            .collect::<Result<Vec<u64>, String>>()
            .expect("Expected successful parsing");

        // then
        assert_eq!(results, &[51, 46, 1445, 669060, 23340]);
    }
}
