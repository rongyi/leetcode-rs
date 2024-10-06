struct Solution;

impl Solution {
    // About the problems - sum of contiguous subarray , prefix sum is a common technique.
    // Another thing is if sum[0, i] % K == sum[0, j] % K, sum[i + 1, j] is divisible by by K.
    // So for current index j, we need to find out how many index i (i < j) exist that has the same mod of K.
    // Now it easy to come up with HashMap <mod, frequency>

    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; k as usize];
        cnt[0] = 1;
        let mut prefix = 0;
        let mut ret = 0;
        for num in nums.into_iter() {
            prefix = (prefix + num % k + k) % k;
            ret += cnt[prefix as usize];
            cnt[prefix as usize] += 1;
        }

        return ret;
    }
}

fn main() {}
