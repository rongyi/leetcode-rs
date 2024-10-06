struct Solution;

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|x, y| x.abs().cmp(&y.abs()));

        nums.into_iter().map(|x| x * x).collect()
    }
}

fn main() {}
