struct Solution;

impl Solution {
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
