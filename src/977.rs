struct Solution;

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|x| x.abs());

        nums.into_iter().map(|x| x * x).collect()
    }
}

fn main() {}
