use crate::exercises::ch02::prac2_2::FunStack;

pub fn calc_rpn(expression: &str) -> f64 {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    let mut main_stack = FunStack::<f64>::new();

    for token in tokens {
        if let Ok(n) = token.parse::<f64>() {
            main_stack = main_stack.push(n);
        } else {
            if main_stack.len() < 2 {
                panic!("Invalid expression: not enough operands for '{}'", token);
            }

            let (num1, stack1) = main_stack.pop();
            let (num2, stack2) = stack1.pop();

            let result = match token {
                "+" => num2.unwrap() + num1.unwrap(),
                "-" => num2.unwrap() - num1.unwrap(),
                "*" => num2.unwrap() * num1.unwrap(),
                "/" => {
                    if num1.unwrap() == 0.0 {
                        panic!("Invalid expression: division by zero");
                    }
                    num2.unwrap() / num1.unwrap()
                },
                _ => panic!("Invalid expression: Unexpected operation '{}'", token),
            };
            main_stack = stack2.push(result);
        }
    }

    if main_stack.len() == 1 {
        main_stack.pop().0.unwrap()
    } else {
        panic!("Invalid expression: too many operands remaining");
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