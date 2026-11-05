#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        // Recursive solution: f(n, k) = (f(n-1, k) + k - 1) % n + 1
        // We solve this iteratively
        // Explanation of the formula (f(n, k) = (f(n-1, k) + k - 1) % n + 1):
        // In this problem, we're doing a circular elimination process (Josephus problem).
        // - f(n, k) represents the position of the winner in a game with n people and counting out every kth person
        // - When we eliminate a person and reduce to n-1 people, the positions get renumbered
        // - The formula connects the solution for n people to the solution for n-1 people
        // - The "+k-1" represents counting k steps from the previous position
        // - The "%n" handles the circular nature of the problem
        // - The "+1" adjusts for 1-indexed positions
        // For example, with n=5, k=2:
        // Start with people numbered 1,2,3,4,5
        // First elimination: 2 is out, remaining: 1,3,4,5
        // Renumber positions: 1→1, 3→2, 4→3, 5→4
        // Second elimination: 4 is out, remaining: 1,3,5
        // Renumber positions: 1→1, 3→2, 5→3
        // Third elimination: 1 is out, remaining: 3,5
        // Renumber positions: 3→1, 5→2
        // Fourth elimination: 5 is out, winner: 3
        let mut winner = 1; // When n=1, winner is 1

        // Start from n=2 and work our way up to the desired n
        for i in 2..=n {
            // Calculate new winner position
            winner = (winner + k - 1) % i + 1;
        }

        winner
    }
}

fn main() {}
