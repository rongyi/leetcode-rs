struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut ret = -1;
        for i in 0..sz {
            for j in i + 1..sz {
                if nums[j] > nums[i] {
                    ret = ret.max(nums[j] - nums[i]);
                }
            }
        }
        ret
    }
}

fn main() {}
