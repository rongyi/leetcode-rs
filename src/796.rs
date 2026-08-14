struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s.is_empty() {
            return true;
        }

        // Check each possible rotation
        for i in 0..s.len() {
            let rotated = format!("{}{}", &s[i..], &s[..i]);
            if rotated == goal {
                return true;
            }
        }

        false
    }
}

fn main() {}
