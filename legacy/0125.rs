struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>();
        let s = s.to_lowercase().chars().collect::<Vec<_>>();

        let mut i = 0;
        let mut j = s.len() as i32 - 1;

        // in case usize minus
        while i < j {
            if s[i as usize] != s[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}

fn main() {
    Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
}
