struct Solution;

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        // collect every 1
        for &num in nums.iter() {
            ret |= num;
        }

        ret
    }
}

fn main() {}
