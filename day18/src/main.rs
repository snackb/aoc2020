use aoc_input::*;

enum Op {
    Add(i64),
    Mult(i64),
    Paren,
}

fn lex(input: &str) -> Vec<char> {
    input.chars().filter(|x| !x.is_whitespace()).collect()
}

fn eval(expression: &[char]) -> i64 {
    let mut stack = Vec::new();
    let mut acc = 0;
    for c in expression {
        match c {
            x if x.is_ascii_digit() => {
                let x = x.to_digit(10).unwrap() as i64;
                match stack.last() {
                    Some(Op::Add(y)) => {
                        acc = x + *y;
                        stack.pop();
                    }
                    Some(Op::Mult(y)) => {
                        acc = x * *y;
                        stack.pop();
                    }
                    Some(Op::Paren) | None => {
                        acc = x;
                    }
                }
            }
            '+' => {
                stack.push(Op::Add(acc));
            }
            '*' => {
                stack.push(Op::Mult(acc));
            }
            '(' => {
                stack.push(Op::Paren);
            }
            ')' => {
                stack.pop().unwrap();
                match stack.last() {
                    Some(Op::Add(y)) => {
                        acc += *y;
                        stack.pop();
                    }
                    Some(Op::Mult(y)) => {
                        acc *= *y;
                        stack.pop();
                    }
                    Some(Op::Paren) | None => {}
                }
            }
            _ => panic!(),
        }
    }
    acc
}

fn eval_with_prec(expression: &[char]) -> i64 {
    let (lhs, expression) = eval_primary(expression);
    eval_with_prec_inner(expression, lhs, 0).0
}

fn eval_with_prec_inner(mut expression: &[char], mut lhs: i64, prec: usize) -> (i64, &[char]) {
    while let Some(mut la) = expression.first() {
        if operator_precedence(la) < prec {
            break;
        }
        let op = la;
        expression = &expression[1..];
        let (mut rhs, expr) = eval_primary(expression); 
        expression = expr;
        if expression.first().is_some() {
            la = &expression[0];
            while operator_precedence(la) > operator_precedence(op) {
                let (trhs, expr) = eval_with_prec_inner(expression, rhs, operator_precedence(la));
                rhs = trhs;
                expression = expr;
                la = if let Some(x) = expression.first() {
                    x
                } else {
                    break;
                };
            }
        }
        match op {
            '+' => lhs += rhs,
            '*' => lhs *= rhs,
            _ => panic!("not an operator"),
        };
    }
    (lhs, expression)
}

fn eval_primary(expression: &[char]) -> (i64, &[char]) {
    match expression.first() {
        Some('(') => {
            let index = get_paren_index(&expression[1..]);
            let val = eval_with_prec(&expression[1..index + 1]);
            (val, &expression[index + 2..])
        }
        Some(x) if x.is_ascii_digit() => (x.to_digit(10).unwrap() as i64, &expression[1..]),
        _ => panic!("not a primary"),
    }
}

fn operator_precedence(operator: &char) -> usize {
    match operator {
        '+' => 2,
        '*' => 1,
        _ => panic!("Not an operator"),
    }
}

fn get_paren_index(mut expr: &[char]) -> usize {
    let mut depth = 1;
    let mut index = 0;
    loop {
        match expr.first() {
            Some('(') => depth += 1,
            Some(')') => depth -= 1,
            None => panic!("unpatched paren"),
            _ => (),
        };
        if depth == 0 {
            return index;
        }
        expr = &expr[1..];
        index += 1;
    }
}

fn main() {
    let input = get_input_txt();
    let evaluated_lines = input.lines().map(|x| eval(&lex(x)));
    println!("Part one: {}", evaluated_lines.sum::<i64>());
    let evaluated_lines = input.lines().map(|x| eval_with_prec(&lex(x)));
    println!("Part two: {}", evaluated_lines.sum::<i64>());
}

#[test]
fn eval_simple() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(231, eval_with_prec(&lex(input)))
}

#[test]
fn eval_complex() {
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(23340, eval_with_prec(&lex(input)))
}
