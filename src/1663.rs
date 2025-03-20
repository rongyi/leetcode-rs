#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut n = n as usize;
        let mut k = k as i32;
        let mut result = vec!['a'; n];

        // We start by setting all chars to 'a', which means we've used n points
        k -= n as i32;

        // Now we go from right to left, trying to put the largest possible char
        for i in (0..n).rev() {
            // Maximum value we can assign to current position (25 moves from 'a' to 'z')
            let max_possible = std::cmp::min(25, k);

            result[i] = (result[i] as u8 + max_possible as u8) as char;
            k -= max_possible;

            if k == 0 {
                break;
            }
        }

        result.into_iter().collect()
    }
}
fn main() {}
