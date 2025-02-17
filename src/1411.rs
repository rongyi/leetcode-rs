#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        if n == 1 {
            return 12;
        }
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        // using three colors
        let mut c3: Vec<i64> = vec![0; n + 1];
        // using two colors
        let mut c2: Vec<i64> = vec![0; n + 1];
        c2[1] = 6;
        c3[1] = 6;

        // https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/solutions/574932/simple-solution-with-picture-follow-up-with-m-colors-english-and/
        for i in 2..=n {
            // if using three colors
            c3[i] = (2 * c2[i - 1] + 2 * c3[i - 1]) % MOD;
            // if using two colors
            c2[i] = (3 * c2[i - 1] + 2 * c3[i - 1]) % MOD;
        }

        ((*c3.last().unwrap() + *c2.last().unwrap()) % MOD) as i32
    }
}
fn main() {}
