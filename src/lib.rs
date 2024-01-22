use std::fmt::Debug;
use std::fmt::Formatter;
use std::vec;

enum Token {
    Number(u16),
    Operator(char, u8),
    Function(String),
    Variable
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Token::Number(num) => write!(f, "{}", num.to_string()),
            Token::Operator(ch, _p) => write!(f, "{}", ch.to_string()),
            Token::Function(fun) => write!(f, "{}", fun),
            Token::Variable => write!(f, "x"),
        }
    }
}

fn materialize_function(alph_str: &mut String, postfix: &mut Vec<Token>, operators: &mut Vec<Token>) {
    if !alph_str.is_empty() {
        if alph_str == "x" {
            postfix.push(Token::Variable);
        }
        else {
            operators.push(Token::Function(alph_str.clone()));
        }
        alph_str.clear();
    }
}

fn materialize_number(num_str: &mut String, postfix: &mut Vec<Token>) {
    if let Ok(num) = num_str.parse() {
        postfix.push(Token::Number(num));
    }
    num_str.clear();
}

fn parse(expression: &str) -> Vec<Token> {
    let mut postfix = Vec::new();
    let mut operators = Vec::new();
    let mut num_str = String::new();
    let mut alph_str = String::new();
    for t in expression.chars() {
        match t {
            '0'..='9' => {
                materialize_function(&mut alph_str, &mut postfix, &mut operators);
                num_str.push(t);
            },
            'a'..='z' => {
                materialize_number(&mut num_str, &mut postfix);
                alph_str.push(t);
            },
            '*' | '/' | '+' | '-' => {
                materialize_number(&mut num_str, &mut postfix);
                materialize_function(&mut alph_str, &mut postfix, &mut operators);
                let mut precedence = 2;
                while precedence > 1 {
                    if let Some(operator) = operators.last() {
                        precedence = match operator { 
                            Token::Operator(_literal, 2) => {
                                if let Some(op) = operators.pop() { postfix.push(op); } else { break; }
                                2
                            },
                            Token::Function(_fun) => {
                                if let Some(op) = operators.pop() { postfix.push(op); } else { break; }
                                2
                            },
                            _ => 0
                        };
                    }
                    else { break; }
                }
                let operator = match t {
                    '*' | '/' => Token::Operator(t, 2),
                    '+' | '-' => Token::Operator(t, 1),
                    _ => panic!("non-defined operator")
                };
                operators.push(operator);
            },
            '(' => {
                materialize_number(&mut num_str, &mut postfix);
                materialize_function(&mut alph_str, &mut postfix, &mut operators);
                operators.push(Token::Operator(t, 0));
            },
            ')' => {
                materialize_number(&mut num_str, &mut postfix);
                materialize_function(&mut alph_str, &mut postfix, &mut operators);
                let mut literal = '\\';
                while literal != '(' {
                    if let Some(operator) = operators.last() {
                        literal = match operator { 
                            Token::Operator('(', _precedence) => {
                                operators.pop();
                                '('
                            },
                            Token::Operator(ch, _precedence) => {
                                let ch = ch.clone();
                                if let Some(op) = operators.pop() {
                                    postfix.push(op);
                                }
                                ch
                            },
                            _ => '\\'
                        };
                    }
                    else { break; }
                }
            },
            _ => panic!("symbol '{}' is not supported", t)
        }
    }
    materialize_number(&mut num_str, &mut postfix);
    materialize_function(&mut alph_str, &mut postfix, &mut operators);
    while let Some(op) = operators.pop() {
        postfix.push(op);
    }
    postfix
}

fn get_result(x: f64, postfix: Vec<Token>) -> f64 {
    let mut stack:Vec<f64> = vec![];
    for token in postfix {
        match token {
            Token::Variable => stack.push(x),
            Token::Number(num) => stack.push(num as f64),
            Token::Operator(op, _precedence) => {
                let right = stack.pop().expect("Right operand should exist!");
                let left = stack.pop().expect("Left operand should exist!");
                match op {
                    '+' => stack.push(left + right),
                    '-' => stack.push(left - right),
                    '*' => stack.push(left * right),
                    '/' => stack.push(left / right),
                    _ => panic!("symbol '{}' is not supported", op)
                };
            },
            Token::Function(fun) => {
                let operand = stack.pop().expect("Operand should exist!") as f64;
                match fun.as_str() {
                    "abs" => stack.push(operand.abs()),
                    "sin" => stack.push(operand.sin()),
                    "cos" => stack.push(operand.cos()),
                    "ln" => stack.push(operand.ln()),
                    "exp" => stack.push(operand.exp()),
                    "sqrt" => stack.push(operand.sqrt()),
                    _ => panic!("unrecognized function!")
                }
            }
        }
    }
    stack.pop().expect("Result of the calculation cannot be returned!")
}

pub fn calculate(x: f64, expression: &str) -> f64 {
    let tokens = parse(expression);
    let res = get_result(x, tokens);
    res
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn calculator_test() {
        let mut variations:HashMap<&str, (&str, f64)> = HashMap::new();
        variations.insert("3+x-2", ("[3, x, 2, -, +]", 5_f64));
        variations.insert("3+x*2", ("[3, x, 2, *, +]", 11_f64));
        variations.insert("x+18/(9-3)", ("[x, 18, 9, 3, -, /, +]", 7_f64));
        variations.insert("3+x*2/(1-5)", ("[3, x, 2, *, 1, 5, -, /, +]", 1_f64));
        variations.insert("3+x*sin(0)", ("[3, x, 0, sin, *, +]", 3_f64));
        variations.insert("cos(2*x)-1/3", ("[2, x, *, cos, 1, 3, /, -]", (2_f64*4_f64).cos()-1_f64/3_f64));
        
        for v in variations {
            let postfix = parse(v.0);
            assert_eq!(format!("{:?}", postfix), v.1.0);
            let result = get_result(4_f64, postfix);
            assert_eq!(result, v.1.1);
        }
    }
}
