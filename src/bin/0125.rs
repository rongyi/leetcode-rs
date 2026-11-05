struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s: String = s.to_ascii_lowercase();
        s.retain(|c| c.is_alphanumeric());
        for (c1, c2) in s.chars().zip(s.chars().rev()) {
            if c1 != c2 {
                return false;
            }
        }

        true
    }
}

fn main() {}
