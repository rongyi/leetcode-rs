struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut nums: Vec<i32> = (1..=n).collect();
        nums.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

        nums
    }
}

fn main() {}
