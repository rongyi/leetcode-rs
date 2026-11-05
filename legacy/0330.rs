struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut miss = 1i64;
        let mut ret = 0;
        let mut i = 0;
        let sz = nums.len();
        while miss <= n as i64 {
            if i < sz && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
