struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_val = 0;
        let mut max_diff = 0;
        let mut ret = 0;

        for &num in nums.iter() {
            let n = num as i64;
            ret = ret.max(max_diff * n);
            max_diff = max_diff.max(max_val - n);

            max_val = max_val.max(n);
        }

        ret
    }
}

fn main() {}
