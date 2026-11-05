#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        // Solve the quadratic equation k * (k + 1) / 2 >= n
        // where k is the number of moves needed
        // This comes from the idea that each move gives us information about
        // drop results at floors 1, 2, ..., k

        let mut k = 0;
        while k * (k + 1) / 2 < n {
            k += 1;
        }
        k
    }
}

fn main() {}
