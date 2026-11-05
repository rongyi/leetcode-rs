#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given an array of locations, a start and finish index, and a specific amount of fuel,
    /// this function calculates the total number of ways to travel from the start location to
    /// the finish location, without running out of fuel.
    ///
    /// The cost of moving from one location to another is the absolute difference of the locations' values.
    /// We can visit locations multiple times, and each visit counts as a new route.
    ///
    /// The function returns the count modulo 10^9 + 7.
    ///
    /// # Parameters
    /// * `locations`: A vector of integers representing positions
    /// * `start`: The index of the starting location
    /// * `finish`: The index of the destination location
    /// * `fuel`: The initial amount of fuel available
    ///
    /// # Returns
    /// The number of different routes from start to finish with the given fuel, modulo 10^9 + 7
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let start = start as usize;
        let finish = finish as usize;
        let fuel = fuel as usize;

        // dp[i][j] = number of ways to reach location i with j fuel left
        let mut dp = vec![vec![0; fuel + 1]; n];

        // Base case: If we're at the finish location, that's 1 valid way
        for j in 0..=fuel {
            dp[finish][j] = 1;
        }

        // For each amount of fuel
        for f in 0..=fuel {
            // For each location
            for i in 0..n {
                // Try to go to every other location
                // from j -> i
                for j in 0..n {
                    if i == j {
                        continue;
                    }

                    let distance = (locations[i] - locations[j]).abs() as usize;
                    if f >= distance {
                        dp[i][f] = (dp[i][f] + dp[j][f - distance]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[start][fuel] as i32
    }
}

fn main() {}
