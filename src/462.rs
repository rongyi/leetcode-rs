struct Solution;

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let sz = nums.len();
        let mid = nums[sz / 2];
        let mut ret = 0;
        for num in nums.into_iter() {
            ret += (num - mid).abs();
        }

        ret
    }
}

fn main() {}
