#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n = scores.len();

        // Create pairs of (age, score) and sort them
        let mut players: Vec<(i32, i32)> = ages.into_iter().zip(scores.into_iter()).collect();

        // Sort by age and then by score
        players.sort_unstable();

        // dp[i] = maximum score attainable with players[0...i] and player i included
        let mut dp = vec![0; n];

        for i in 0..n {
            // Current player's score
            let (_, score_i) = players[i];
            // Initially, assume we can at least have the current player's score
            dp[i] = score_i;

            // Try to add current player to previous teams
            for j in 0..i {
                let (_, score_j) = players[j];
                // We can add current player to team j if score_i >= score_j
                // (we've already sorted by age, so age_i >= age_j)
                if score_i >= score_j {
                    dp[i] = dp[i].max(dp[j] + score_i);
                }
            }
        }

        // Return the maximum score among all possible teams
        *dp.iter().max().unwrap_or(&0)
    }
}

fn main() {}
