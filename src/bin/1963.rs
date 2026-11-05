struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut balance = 0;
        let mut swaps = 0;
        for c in s.chars() {
            if c == '[' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                swaps += 1;
                balance = 1;
            }
        }

        swaps
    }
}
fn main() {}
