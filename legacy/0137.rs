struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut uniq = 0;

        for i in 0..32 {
            let mut sum = 0;
            for num in &nums {
                let c = (num >> i) & 0x1;
                sum = (sum + c) % 3;
            }

            uniq |= sum << i;
        }

        uniq
    }
}

fn main() {}
