#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut insertions = 0;
        let mut open_count = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        // eliminate all right )
        while i < chars.len() {
            if chars[i] == '(' {
                open_count += 1;
            } else {
                // encountered ')'
                if i + 1 < chars.len() && chars[i + 1] == ')' {
                    // Found '))' pair
                    if open_count > 0 {
                        open_count -= 1;
                    } else {
                        // Need to insert an opening parenthesis
                        insertions += 1;
                    }
                    i += 1; // Skip the next ')'
                } else {
                    // Found a single ')', but we need '))'
                    if open_count > 0 {
                        // Need to insert one more ')'
                        insertions += 1;
                        open_count -= 1;
                    } else {
                        // Need to insert '(' and ')'
                        insertions += 2;
                    }
                }
            }
            i += 1;
        }

        // For remaining open parentheses, we need to add two closing parentheses each
        insertions += open_count * 2;

        insertions
    }
}
fn main() {}
