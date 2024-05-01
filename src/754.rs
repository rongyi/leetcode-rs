#![allow(dead_code)]
struct Solution;

impl Solution {
    // 1 + 2 + 3 + 4 .. + N = N(N+1) / 2
    // Choose some of the numbers to be negatives.
    // Take 3 and 4 for example
    // 1 + 2 - 3 - 4 .. + N = N(N+1) / 2 - 3 - 4 - 3 - 4 (Note: subtract twice)
    // We want this number to be equal to our target.
    // Thus,
    // N(N+1)/2 - 2 * x = target
    // N(N+1) - 2 * target = 4 * x
    // Here x can be any number since we can choose form 1 to N, and any other combinations
    // The problem becomes find minimal i that
    // (i(i+1) - 2 * target) MOD 4 = 0, where i <= N and i(i+1) >= 2 * target
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        let mut i = (2.0 * target as f64).sqrt() as i32;
        loop {
            let remain = i * i + i - 2 * target;
            if remain >= 0 && remain % 4 == 0 {
                return i;
            }
            i += 1;
        }
    }
}

fn main() {}
