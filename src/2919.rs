struct Solution;

impl Solution {
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        // dp1, dp2, dp3 represent the min cost where the beautiful
        // element was 1, 2, or 3 positions ago.
        let mut dp1 = 0i64;
        let mut dp2 = 0i64;
        let mut dp3 = 0i64;

        for &val in nums.iter() {
            // Cost to make the CURRENT element the "beautiful" one
            let cost_to_increase = 0.max(k - val as i64);

            // The current beautiful element can build upon the best
            // of the last three beautiful elements.
            let current_dp = cost_to_increase + dp1.min(dp2).min(dp3);

            // Shift states: current becomes the new "1 position ago"
            dp3 = dp2;
            dp2 = dp1;
            dp1 = current_dp;
        }

        // To satisfy the very last windows in the array, the
        // beautiful element must be within the last 3 positions.
        dp1.min(dp2).min(dp3)
    }
}
fn main() {}
