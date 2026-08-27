// Reverse Polish notation
// Used in the implementation of stack operations in the CPU
// https://www.youtube.com/watch?v=7ha78yWRDlE
// https://www.youtube.com/watch?v=qN8LPIcY6K4
// leetcode problem: https://leetcode.com/problems/evaluate-reverse-polish-notation/description/?envType=problem-list-v2&envId=dsa-linear-shoal-stack

use std::ops::{Add, Sub};

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    for token in tokens {
        match token.as_str() {
            "+" => {
                let op_2 = stack.pop().unwrap_or(0);
                let op_1 = stack.pop().unwrap_or(0);

                stack.push(op_1.add(op_2));
            }

            "-" => {
                let op_2 = stack.pop().unwrap_or(0);
                let op_1 = stack.pop().unwrap_or(0);

                stack.push(op_1.sub(op_2));
            }

            "*" => {
                let op_2 = stack.pop().unwrap_or(0);
                let op_1 = stack.pop().unwrap_or(0);

                stack.push(op_1.checked_mul(op_2).unwrap_or(0));
            }

            "/" => {
                let op_2 = stack.pop().unwrap_or(0);
                let op_1 = stack.pop().unwrap_or(0);

                stack.push(op_1.checked_div(op_2).unwrap_or(0));
            }
            _ => stack.push(token.parse().unwrap_or(0)),
        }
    }

    stack[0]
}
fn main() {
    let tokens = vec!["2", "1", "+", "3", "*"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{}", eval_rpn(tokens));
}

#[cfg(test)]
mod test {
    use crate::eval_rpn;

    #[test]
    fn test_eval_rpn_1() {
        assert_eq!(
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        )
    }
    #[test]
    fn test_eval_rpn_2() {
        assert_eq!(
            eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string(),
            ]),
            22
        )
    }
}
