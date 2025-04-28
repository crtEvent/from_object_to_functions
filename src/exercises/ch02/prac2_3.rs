#![allow(unused)]

use crate::exercises::ch02::prac2_2::FunStack;

pub fn calc_rpn(expression: &str) -> f64 {
    let main_stack = FunStack::new();

    let (result, stack) = expression
        .split_whitespace()
        .fold(main_stack, |stack, token| reduce(stack, token))
        .pop();

    if stack.len() >= 1 {
        panic!("Invalid expression: too many operands remaining");
    }

    result.unwrap()
}

fn operations_map(op: &str) -> fn(f64, f64) -> f64 {
    match op {
        "+" => |x, y| x + y,
        "-" => |x, y| x - y,
        "*" => |x, y| x * y,
        "/" => |x, y| {
            if y == 0.0 {
                panic!("Invalid expression: division by zero")
            }
            x / y
        },
        _ => panic!("Invalid expression: Unexpected operation '{}'", op),
    }
}

fn reduce(stack: FunStack<f64>, token: &str) -> FunStack<f64> {
    if let Ok(n) = token.parse::<f64>() {
        stack.push(n)
    } else {
        let op: fn(f64, f64) -> f64 = operations_map(token);

        if stack.len() < 2 {
            panic!("Invalid expression: not enough operands for '{}'", token);
        }

        let (num1, stack1) = stack.pop();
        let (num2, stack2) = stack1.pop();
        stack2.push(op(num2.unwrap(), num1.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_rpn() {
        assert_eq!(calc_rpn("4 5 +"), 9.0);
        assert_eq!(calc_rpn("6 2 /"), 3.0);
        assert_eq!(calc_rpn("5 6 2 1 + / *"), 10.0);
        assert_eq!(calc_rpn("2 5 * 4 + 3 2 * 1 + /"), 2.0);
    }

    #[test]
    #[should_panic(expected = "Invalid expression: not enough operands for '*'")]
    fn test_not_enough_operands() {
        calc_rpn("4 *");
    }

    #[test]
    #[should_panic(expected = "Invalid expression: division by zero")]
    fn test_division_by_zero() {
        calc_rpn("4 0 /");
    }

    #[test]
    #[should_panic(expected = "Invalid expression: Unexpected operation '&'")]
    fn test_unexpected_operation() {
        calc_rpn("4 5 &");
    }

    #[test]
    #[should_panic(expected = "Invalid expression: too many operands remaining")]
    fn test_too_many_operands_remaining() {
        calc_rpn("4 4 5 +");
    }
}
