#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        Self::check_can_form_palindrome(a_bytes, b_bytes)
            || Self::check_can_form_palindrome(b_bytes, a_bytes)
    }

    fn check_can_form_palindrome(a: &[u8], b: &[u8]) -> bool {
        if a.len() <= 1 {
            return true;
        }

        let (mut left, mut right) = (0, a.len() - 1);

        // Check if a_prefix + b_suffix can form a palindrome
        while left < right && a[left] == b[right] {
            left += 1;
            right -= 1;
        }

        // Check if we've already matched all characters
        // If left >= right, we've matched the entire string and have a palindrome
        if left >= right {
            return true;
        }

        // At this point, a_prefix[0...left-1] matched with b_suffix[right+1...n-1]
        // So we need to check if the remaining middle portion is a palindrome
        // either from a[left...right] or b[left...right]
        // Check if remaining substring a[left..=right] is a palindrome
        if Self::is_palindrome(a, left, right) || Self::is_palindrome(b, left, right) {
            return true;
        }

        false
    }

    fn is_palindrome(s: &[u8], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
fn main() {}
