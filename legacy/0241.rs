struct Solution;

use std::collections::HashMap;
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
}

impl Operator {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match self {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let tokens: Vec<char> = expression.chars().collect();

        fn recur(tokens: &[char], cache: &mut HashMap<Vec<char>, Vec<i32>>) -> Vec<i32> {
            if let Some(result) = cache.get(tokens) {
                return result.clone();
            }
            let mut ret = Vec::new();

            for i in 0..tokens.len() {
                if tokens[i] == '+' || tokens[i] == '-' || tokens[i] == '*' {
                    let left = recur(&tokens[0..i], cache);
                    let right = recur(&tokens[i + 1..], cache);
                    for &l in &left {
                        for &r in &right {
                            let value = Operator::from_char(tokens[i]).apply(l, r);
                            ret.push(value);
                        }
                    }
                }
            }
            // the number case
            if ret.is_empty() {
                ret.push(tokens.iter().collect::<String>().parse().unwrap());
            }

            ret
        }

        let mut cache = HashMap::new();

        recur(&tokens, &mut cache)
    }
}

fn main() {}
