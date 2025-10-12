struct Solution;

impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        stones
            .windows(3)
            .fold(stones[1] - stones[0], |acc, cur| acc.max(cur[2] - cur[0]))
    }
}

fn main() {}
