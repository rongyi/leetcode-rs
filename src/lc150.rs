struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut expr = Vec::new();
        for token in &tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let op2 = expr.pop().unwrap();
                    let op1 = expr.pop().unwrap();
                    if token == "+" {
                        expr.push(op1 + op2);
                    } else if token == "-" {
                        expr.push(op1 - op2);
                    } else if token == "*" {
                        expr.push(op1 * op2);
                    } else {
                        expr.push(op1 / op2);
                    }
                }
                s @ _ => expr.push(s.parse::<i32>().unwrap()),
            }
        }

        expr.pop().unwrap()
    }
}

fn main() {}
