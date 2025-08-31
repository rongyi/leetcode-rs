struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        // zero can be put anywhere ?
        // if zero between one_count, we can just add 0 there
        // and suffix and prefix 0 can be put as well
        let zero_cout = s.chars().filter(|c| *c == '0').count();
        let mut val = 0;
        let mut pow = 1;
        let mut one_count = 0;
        for c in s.chars().rev() {
            if val + pow > k {
                break;
            }
            if c == '1' {
                val += pow;
                one_count += 1;
            }
            pow <<= 1;
        }
        (zero_cout + one_count) as _
    }
}

fn main() {}
