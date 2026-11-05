#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }

        // prefix sums for odd and even indexed elements
        let mut odd_sum = vec![0; n + 1];
        let mut even_sum = vec![0; n + 1];

        for i in 0..n {
            odd_sum[i + 1] = odd_sum[i];
            even_sum[i + 1] = even_sum[i];

            if i % 2 == 0 {
                even_sum[i + 1] += nums[i];
            } else {
                odd_sum[i + 1] += nums[i];
            }
        }

        let mut count = 0;

        for i in 0..n {
            // Calculate sum of odd-indexed elements after removal
            // Elements before i keep their parity
            // Elements after i flip their parity (odd becomes even, even becomes odd)
            let odd_before = odd_sum[i];
            let even_before = even_sum[i];

            // Elements after i (excluding i)
            let odd_after = even_sum[n] - even_sum[i + 1];
            let even_after = odd_sum[n] - odd_sum[i + 1];

            // Total new sums
            let new_odd_sum = odd_before + odd_after;
            let new_even_sum = even_before + even_after;

            if new_odd_sum == new_even_sum {
                count += 1;
            }
        }

        count
    }
}

fn main() {}
