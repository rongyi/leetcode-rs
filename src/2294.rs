struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let sz = nums.len();
        let mut ret = 0;

        let mut i = 0;
        while i < sz {
            let mut j = i;
            while j < sz && nums[j] - nums[i] <= k {
                j += 1;
            }
            i = j;
            ret += 1;
        }
        ret
    }
}

fn main() {}
