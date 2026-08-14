struct Solution;

impl Solution {
    /// LeetCode 813: Largest Sum of Averages
    ///
    /// Partition nums into at most k contiguous groups to MAXIMIZE
    /// the sum of each group's average.
    ///
    /// dp[i][k] = best sum-of-averages for the first i elements
    ///            split into exactly k groups.
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let sz = nums.len();
        let mut prefix = vec![0; sz + 1];
        for (i, &num) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + num;
        }
        if k <= 1 {
            return prefix[sz] as f64 / sz as f64;
        }
        let k = k as usize;
        let mut dp = vec![vec![0.0; k + 1]; sz + 1];

        // Base: only one group → average of the first i elements.
        for i in 1..=sz {
            dp[i][1] = prefix[i] as f64 / i as f64;
        }
        let max_k = k;

        // Build up group count from 2 to K.
        for groups in 2..=max_k {
            // i = total elements considered; need at least `groups` elements
            // (one per group), so i starts at `groups`.
            for i in groups..=sz {
                // j = number of elements consumed by the FIRST `groups-1` groups.
                // The last group is nums[j..i].
                //
                //   j must be >= groups-1 : each earlier group needs >= 1 element
                //   j must be <= i-1     : the last group needs >= 1 element
                //
                // For each valid split point j:
                //   best(j, i) = dp[j][groups-1]  (first j elements, groups-1 groups)
                //              + avg(nums[j..i])  (the k-th/last group)
                // Take the max over all j.
                for j in (groups - 1..=i - 1).rev() {
                    // Average of the last group nums[j..i].
                    let last_avg = (prefix[i] - prefix[j]) as f64 / (i - j) as f64;
                    dp[i][groups] = dp[i][groups].max(dp[j][groups - 1] + last_avg);
                }
            }
        }

        dp[sz][k]
    }
}

fn main() {
    let tests = [
        (vec![9, 1, 2, 3, 9], 3, 20.0),
        (vec![1, 2, 3, 4, 5, 6, 7], 4, 20.5),
        (vec![4, 1, 7, 3], 3, 12.5),
    ];

    for (nums, k, expected) in &tests {
        let result = Solution::largest_sum_of_averages(nums.clone(), *k);
        let pass = (result - expected).abs() < 1e-9;
        println!(
            "{} nums={:?} k={} → {} (expected {})",
            if pass { "✓" } else { "✗" },
            nums,
            k,
            result,
            expected
        );
    }
}
