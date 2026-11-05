struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let sz = s.len();
        for i in 1..sz / 2 + 1 {
            if sz % i == 0 {
                let chunk = s[..i].to_string();
                let cur = chunk.repeat(sz / i);
                if cur == s {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {}
