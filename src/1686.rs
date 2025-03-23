#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        // // Algorithm:
        // 1. Create a combined value for each stone (Alice's value + Bob's value)
        // 2. Sort the stones by their combined value in descending order
        // 3. Alice takes stones at even indices (0th, 2nd, etc.)
        // 4. Bob takes stones at odd indices (1st, 3rd, etc.)
        // 5. Compare their scores to determine the winner
        //
        // Intuition: Both players want to maximize their score while preventing
        // the other from scoring high. The optimal strategy is to pick stones
        // with highest combined value first, which maximizes your gain while
        // minimizing opponent's opportunity.
        // Calculate the combined value for each stone
        let mut combined_values: Vec<(i32, usize)> = alice_values
            .iter()
            .zip(bob_values.iter())
            .enumerate()
            .map(|(i, (&a, &b))| (a + b, i))
            .collect();

        // Sort by combined value in descending order
        combined_values.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut alice_score = 0;
        let mut bob_score = 0;

        // Alice and Bob take turns picking stones
        for (i, &(_, index)) in combined_values.iter().enumerate() {
            if i % 2 == 0 {
                // Alice's turn
                alice_score += alice_values[index];
            } else {
                // Bob's turn
                bob_score += bob_values[index];
            }
        }

        // Return result based on scores
        if alice_score > bob_score {
            1
        } else if alice_score < bob_score {
            -1
        } else {
            0
        }
    }
}

fn main() {}
