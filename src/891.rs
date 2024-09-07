struct Solution;

impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let mut c = 1;
        let mut ret = 0;
        let m = 1e9 as i64 + 7;

        nums.sort_unstable();

        // For A[i]:
        // There are i smaller numbers,
        // so there are 2 ^ i sequences in which A[i] is maximum.
        // we should do res += A[i] * (2 ^ i)

        // There are n - i - 1 bigger numbers,
        // so there are 2 ^ (n - i - 1) sequences in which A[i] is minimum.
        // we should do res -= A[i] * 2 ^ (n - i - 1)
        for i in 0..nums.len() {
            ret = (ret + nums[i] as i64 * c - nums[nums.len() - i - 1] as i64 * c) % m;
            c = (c << 1) % m;
        }

        ((ret + m) % m) as i32
    }
}

fn main() {}
