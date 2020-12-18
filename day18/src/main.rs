use std::env;
use std::io;

#[derive(Debug, Clone)]
enum LexItem {
    LParen,
    RParen,
    Op(char),
    Num(u64),
}

fn lex(line: &str) -> Result<Vec<LexItem>, String> {
    let mut result = vec![];
    let mut number = String::new();
    for c in line.chars() {
        match c {
            '0'..='9' => number.push(c),

            '+' | '-' | '*' | '/' => result.push(LexItem::Op(c)),
            '(' => result.push(LexItem::LParen),
            ')' => {
                if number.len() > 0 {
                    result.push(LexItem::Num(number.parse::<u64>().unwrap()));
                    number.clear();
                }
                result.push(LexItem::RParen);
            }
            ' ' => {
                if number.len() > 0 {
                    result.push(LexItem::Num(number.parse::<u64>().unwrap()));
                    number.clear();
                }
            }
            _ => return Err(format!("unexpected character {}", c)),
        }
    }
    if number.len() > 0 {
        result.push(LexItem::Num(number.parse::<u64>().unwrap()));
        number.clear();
    }
    return Ok(result);
}

fn rpn(tokens: Vec<LexItem>) -> Vec<LexItem> {
    let mut output = vec![];
    let mut operators = vec![];
    for token in tokens {
        match token {
            LexItem::Num(_) => output.push(token),
            LexItem::Op(_) => {
                loop {
                    if let Some(LexItem::LParen) = operators.last() {
                        break;
                    } else if let None = operators.last() {
                        break;
                    }
                    output.push(operators.pop().unwrap());
                }
                operators.push(token)
            }
            LexItem::LParen => operators.push(token),
            LexItem::RParen => {
                loop {
                    if let Some(LexItem::LParen) = operators.last() {
                        break;
                    } else if let None = operators.last() {
                        break;
                    }
                    output.push(operators.pop().unwrap());
                }
                if let Some(LexItem::LParen) = operators.last() {
                    operators.pop();
                }
            }
        }
    }

    while let Some(_) = operators.last() {
        output.push(operators.pop().unwrap());
    }

    return output;
}

fn solve_rpn(tokens: Vec<LexItem>) -> i64 {
    let mut stack = vec![];
    for token in tokens {
        match token {
            LexItem::Num(n) => stack.push(n as i64),
            LexItem::Op(op) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                match op {
                    '+' => stack.push(a + b),
                    '-' => stack.push(a - b),
                    '*' => stack.push(a * b),
                    '/' => stack.push(a / b),
                    _ => panic!("LexItem:op({}) invalid operator", op),
                }
            }
            _ => panic!("solve_rpn invalid token{:?}", token),
        }
    }

    return stack.pop().unwrap();
}

fn part1(line: &str) -> i64 {
    let tokens = lex(line).unwrap();
    let prep = rpn(tokens);
    return solve_rpn(prep);
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read_to_string failed");
    let mut sum = 0;
    for line in text.lines() {
        sum += part1(line);
    }
    println!("{}", sum);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
