#![allow(dead_code)]
struct Solution;

const TARGET: f64 = 24.0;
const EPSILON: f64 = 0.001;
impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let nums: Vec<f64> = cards.into_iter().map(|v| v as f64).collect();

        Self::solve(&nums)
    }
    fn solve(nums: &Vec<f64>) -> bool {
        if nums.len() == 1 {
            return (nums[0] - TARGET).abs() < EPSILON;
        }

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                // can not use twice or more
                if i == j {
                    continue;
                }
                let mut new_nums = Vec::new();
                for k in 0..nums.len() {
                    if k != i && k != j {
                        new_nums.push(nums[k]);
                    }
                }

                for &op in &['+', '-', '*', '/'] {
                    // i * j == j * i
                    // i + j == j + i
                    // so define an order and make it execute only one branch
                    if (op == '+' || op == '*') && i > j {
                        continue;
                    }
                    if op == '/' && nums[j].abs() < EPSILON {
                        continue;
                    }
                    let ret = match op {
                        '+' => nums[i] + nums[j],
                        '-' => nums[i] - nums[j],
                        '*' => nums[i] * nums[j],
                        '/' => nums[i] / nums[j],
                        _ => unreachable!(),
                    };
                    new_nums.push(ret);
                    if Self::solve(&new_nums) {
                        return true;
                    }

                    new_nums.pop();
                }
            }
        }

        false
    }
}

fn main() {}
