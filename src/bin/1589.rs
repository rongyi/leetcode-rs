#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        // Initialize counts array to track frequency of each position
        let mut counts = vec![0; n];

        // Use line sweep algorithm to calculate frequency
        for req in &requests {
            let start = req[0] as usize;
            let end = req[1] as usize;

            counts[start] += 1;
            if end + 1 < n {
                counts[end + 1] -= 1;
            }
        }

        // Calculate prefix sum to get actual frequency of each position
        for i in 1..n {
            counts[i] += counts[i - 1];
        }

        // Sort both arrays
        counts.sort_unstable_by(|a, b| b.cmp(a)); // Sort counts in descending order
        let mut sorted_nums = nums;
        sorted_nums.sort_unstable_by(|a, b| b.cmp(a)); // Sort nums in descending order

        // Assign the largest numbers to the positions that appear most frequently
        let mut result: i64 = 0;
        for i in 0..n {
            result = (result + (counts[i] as i64 * sorted_nums[i] as i64)) % MOD;
        }

        result as i32
    }
}
fn main() {}
