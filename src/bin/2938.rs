struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut swaps: i64 = 0;
        let mut black_ball_count: i64 = 0;

        for c in s.chars() {
            if c == '1' {
                // We found a black ball, it will block any '0'
                // that appears later in the string.
                black_ball_count += 1;
            } else {
                // We found a white ball ('0').
                // It needs to swap with every black ball seen so far
                // to reach the left side.
                swaps += black_ball_count;
            }
        }

        swaps
    }
}

fn main() {}
