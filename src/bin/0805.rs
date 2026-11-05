struct Solution;

use std::collections::HashSet;

mod ai {
    struct Solution;

    use std::collections::HashSet;
    impl Solution {
        pub fn split_array_same_average(nums: Vec<i32>) -> bool {
            let n = nums.len();
            let total_sum: i32 = nums.iter().sum();

            if n <= 1 {
                return false;
            }

            // Try each possible subset size k
            for k in 1..=n / 2 {
                // Check if total_sum * k is divisible by n
                // this is deduced from math
                // sum_A * (n - k) = (total_sum - sum_A) * k
                // sum_A * n - sum_A * k = total_sum * k - sum_A * k
                // sum_A * n = total_sum * k
                if (total_sum * k as i32) % n as i32 != 0 {
                    continue;
                }

                let target_sum = total_sum * k as i32 / n as i32;

                // DP: dp[size] = HashSet of possible sums with exactly 'size' elements
                let mut dp = vec![HashSet::new(); k + 1];
                dp[0].insert(0); // Empty subset has sum 0

                // Try each number
                for &num in &nums {
                    // IMPORTANT: Go backwards to avoid reusing the same element
                    for size in (1..=k).rev() {
                        // For each sum achievable with (size-1) elements
                        let new_sums: Vec<i32> =
                            dp[size - 1].iter().map(|&sum| sum + num).collect();

                        // Add all new sums to dp[size]
                        for cur_sum in new_sums {
                            // Optimization: Only keep sums <= target_sum
                            if cur_sum <= target_sum {
                                dp[size].insert(cur_sum);
                            }
                        }
                    }
                }

                // Check if we found a subset of size k with sum = target_sum
                if dp[k].contains(&target_sum) {
                    return true;
                }
            }

            false
        }
    }
}

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        let sz = nums.len();
        let m = sz / 2;
        let mut is_possible = false;

        // sum_A / k = (total_sum - sum_A) / (n - k)
        // -> sum_A * n = total_sum * k
        for i in 0..m {
            if is_possible {
                break;
            }
            if sum as usize * (i + 1) % sz == 0 {
                is_possible = true;
            }
        }
        if !is_possible {
            return false;
        }
        let mut sums: Vec<HashSet<i32>> = vec![HashSet::new(); sz];
        sums[0].insert(0);

        for &num in nums.iter() {
            for i in (1..=m).rev() {
                for &t in sums[i - 1].clone().iter() {
                    sums[i].insert(t + num);
                }
            }
        }
        for i in 1..=m {
            let key = (sum as usize * i / sz) as i32;
            if sum as usize * i % sz == 0 && sums[i].contains(&key) {
                return true;
            }
        }

        false
    }
}

fn main() {}
