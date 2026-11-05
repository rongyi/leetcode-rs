struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        for &c in t.iter() {
            if c == s[i] {
                i += 1;
            }
            if i == s.len() {
                return true;
            }
        }

        i == s.len()
    }
}

fn main() {}
