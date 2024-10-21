struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let sz = nums.len();
        let mut ret = 0;
        let mut i = 0;
        let mut cur_flips = 0;
        for j in 0..sz {
            cur_flips += 1 - nums[j];
            while cur_flips > k {
                if nums[i] == 0 {
                    cur_flips -= 1;
                }
                i += 1;
            }
            ret = ret.max((j - i + 1) as i32);
        }

        ret
    }
}

fn main() {}
