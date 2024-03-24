struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }
        let mut total_cnt = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let cur_sum = sum[j + 1] - sum[i];
                if cur_sum == k {
                    total_cnt += 1;
                }
            }
        }
        total_cnt
    }
}

fn main() {}
