struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut cur = 0;
        for &num in nums.iter() {
            if num == 0 {
                cur = 0;
            } else {
                cur += 1;
                ret = ret.max(cur);
            }
        }

        ret
    }
}

fn main() {}
