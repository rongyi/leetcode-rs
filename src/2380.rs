struct Solution;

impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut zeros = 0;
        let mut ret = 0;
        for c in s.chars() {
            if c == '0' {
                zeros += 1;
            } else if zeros > 0 {
                ret = std::cmp::max(ret + 1, zeros);
            }
        }
        ret
    }
}

fn main() {}
