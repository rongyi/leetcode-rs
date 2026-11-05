struct Solution;

impl Solution {
    pub fn find_closest_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| a.abs().cmp(&b.abs()).then(b.cmp(&a)));
        nums[0]
    }
}

fn main() {}
