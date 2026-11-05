struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = usize::MAX;
        let mut sum = 0;
        let mut i = 0;
        for j in 0..nums.len() {
            sum += nums[j];
            while sum >= target {
                min_len = min_len.min(j - i + 1);
                sum -= nums[i];
                i += 1;
            }
        }

        if min_len == usize::MAX {
            return 0;
        }
        return min_len as i32;
    }
}

fn main() {}
