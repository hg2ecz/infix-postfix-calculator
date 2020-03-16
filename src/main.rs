use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Infix {
    Operator(Op),
    Number(f64),
}

pub fn tokenize(in_str: &str) -> Vec<Infix> {
    let mut tokenvec: Vec<Infix> = Vec::new();
    let mut token: String = String::new();
    for c in in_str.chars().filter(|&c| c > ' ') {
        if ['+', '-', '*', '/'].contains(&c) {
            tokenvec.push(Infix::Number(token.parse().unwrap())); // number
            token.clear();
            tokenvec.push(Infix::Operator(match c {
                '+' => Op::Add,
                '-' => Op::Sub,
                '*' => Op::Mul,
                '/' => Op::Div,
                _ => Op::Add, // non-exhaustive patterns ... NEVER run this case
            }));
        } else {
            token.push(c);
        }
    }
    if !token.is_empty() {
        tokenvec.push(Infix::Number(token.parse().unwrap()));
    }
    tokenvec
}

pub struct InstrStack {
    op: Op,
    prec: u8,
}

#[derive(Debug)]
pub enum Postfix {
    Operator(Op),
    Number(f64),
}

fn calc(rpnstack: &mut Vec<f64>, operator: &Op) {
    let r1 = rpnstack.pop().unwrap();
    let r2 = rpnstack.pop().unwrap();
    match operator {
        Op::Add => rpnstack.push(r1 + r2),
        Op::Sub => rpnstack.push(r1 - r2),
        Op::Mul => rpnstack.push(r1 * r2),
        Op::Div => rpnstack.push(r1 / r2),
    }
}

pub fn infix_to_postfix_calc(tokenvec: &[Infix]) -> Option<f64> {
    let mut stack: Vec<InstrStack> = Vec::new(); // instruction stack
    let mut rpnstack: Vec<f64> = Vec::new(); // RPN stack
    let mut postfixdebug: Vec<Postfix> = Vec::new(); // full RPN

    for token in tokenvec {
        match token {
            Infix::Number(value) => {
                rpnstack.push(*value);
                postfixdebug.push(Postfix::Number(*value));
            }
            Infix::Operator(op) => {
                let prec = match op {
                    Op::Add | Op::Sub => 1,
                    Op::Mul | Op::Div => 2,
                };
                if let Some(oldop) = stack.last() {
                    if oldop.prec >= prec {
                        let calc_op = stack.pop().unwrap();
                        calc(&mut rpnstack, &calc_op.op);
                        postfixdebug.push(Postfix::Operator(calc_op.op));
                    }
                }
                stack.push(InstrStack { op: *op, prec });
            }
        }
    }

    while stack.len() > 0 {
        let calc_op = stack.pop().unwrap();
        calc(&mut rpnstack, &calc_op.op);
        postfixdebug.push(Postfix::Operator(calc_op.op));
    }

    println!("Postfixdebug: {:?}", postfixdebug);
    if rpnstack.len() == 1 {
        return rpnstack.pop();
    }
    None
}

fn main() {
    let mut in_string = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut in_string).unwrap();
    println!("Input string: {}", in_string);

    let tokenvec = tokenize(&in_string);
    println!("Tokens: {:?}", tokenvec);

    if let Some(result) = infix_to_postfix_calc(&tokenvec) {
        println!("Result: {}", result);
    } else {
        println!("Hibás feldolgozás");
    }
}
