#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        // In a tournament with n teams, each eliminated team plays exactly one match
        // So the total number of matches is n-1 (one less than the number of teams)
        // Think about it: in a tournament with single elimination,
        // each team except the winner gets eliminated.
        // For a team to be eliminated, they must lose exactly one match.
        // So if n-1 teams get eliminated, there must be n-1 matches total.
        n - 1
    }
}
fn main() {}
