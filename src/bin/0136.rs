struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut val = 0;
        for v in nums.into_iter() {
            val ^= v;
        }

        val
    }
}

fn main() {}
