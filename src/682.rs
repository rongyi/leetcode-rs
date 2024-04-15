#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut scores: Vec<i32> = Vec::new();

        for op in operations.into_iter() {
            match op.as_str() {
                "C" => {
                    scores.pop();
                }
                "D" => {
                    let val = *scores.last().unwrap();
                    scores.push(val * 2);
                }
                "+" => {
                    let num1 = scores[scores.len() - 1];
                    let num2 = scores[scores.len() - 2];
                    scores.push(num1 + num2);
                }
                _ => {
                    let val = op.parse::<i32>().unwrap();
                    scores.push(val);
                }
            }
        }

        scores.into_iter().fold(0, |acc, x| acc + x)
    }
}

fn main() {}
