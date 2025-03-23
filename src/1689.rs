#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        // The min number of strings needed is the maximum digit in the number
        // This is because each Deci-Binary number can contribute at most 1 to each position
        // For example, if we have "32", we need 3 Deci-Binary numbers: "10" + "10" + "10" = "30", plus "1" + "1" = "2"
        n.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .max()
            .unwrap_or(0)
    }
}
fn main() {}
