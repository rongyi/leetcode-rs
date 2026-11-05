struct Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut prefix_sum = 0;
        for (i, &val) in nums.iter().enumerate() {
            prefix_sum += val as i64;
            ret = ret.max((prefix_sum + i as i64) / (i as i64 + 1));
        }
        ret as _
    }
}

fn main() {}
