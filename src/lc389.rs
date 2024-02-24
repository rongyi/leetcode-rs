struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut f = 0;
        for c in s.chars().chain(t.chars()) {
            f ^= c as u8;
        }

        f as char
    }
}

fn main() {
    Solution::find_the_difference("abcd".to_string(), "abcde".to_string());
}
