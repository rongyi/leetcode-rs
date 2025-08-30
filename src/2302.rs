struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(|s| s as i64).collect();
        let mut win_sum = 0i64;
        let sz = nums.len();

        let mut i = 0;
        let mut ret = 0;
        for j in 0..sz {
            win_sum += nums[j];
            while win_sum * (j - i + 1) as i64 >= k {
                win_sum -= nums[i];
                i += 1;
            }
            ret += (j - i + 1) as i64;
        }
        ret
    }
}

fn main() {}
