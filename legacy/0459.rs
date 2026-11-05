struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let sz = s.len();
        for i in 1..sz / 2 + 1 {
            if sz % i == 0 {
                let part = &s[..i];
                let mut cur = String::new();
                while cur.len() < sz {
                    cur.push_str(part);
                }
                if cur == s {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {}
