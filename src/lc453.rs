struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let nums: Vec<i64> = nums.iter().map(|&i| i as i64).collect();
        let a = nums.iter().fold(0i64, |acc, &cur| acc + cur);
        let b = *nums.iter().min().unwrap() * (nums.len() as i64);
        (a - b) as i32
    }
}

fn main() {}
