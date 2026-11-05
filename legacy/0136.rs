struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num = 0;

        for i in nums.iter() {
            num ^= *i;
        }

        num
    }
}

fn main() {}
