#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = events.len();

        // Sort events by start time
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));

        // Create a DP array: dp[i][j] = max value when considering events[i...n-1] and attending at most j events
        let mut dp = vec![vec![-1; k + 1]; n + 1];

        Self::dfs(0, k, &events, &mut dp)
    }

    fn dfs(index: usize, k: usize, events: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
        // Base cases
        if index >= events.len() || k == 0 {
            return 0;
        }

        // If already computed, return the value
        if dp[index][k] != -1 {
            return dp[index][k];
        }

        // Skip the current event
        let skip = Self::dfs(index + 1, k, events, dp);

        // Attend the current event
        let end_time = events[index][1];

        // Binary search to find the next event that can be attended
        let mut low = index + 1;
        let mut high = events.len() - 1;
        let mut next_index = events.len();

        while low <= high {
            let mid = low + (high - low) / 2;
            if events[mid][0] > end_time {
                next_index = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        let attend = events[index][2] + Self::dfs(next_index, k - 1, events, dp);

        // Take the maximum of skipping or attending
        dp[index][k] = skip.max(attend);

        dp[index][k]
    }
}
fn main() {}
