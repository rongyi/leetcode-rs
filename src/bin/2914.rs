struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut changes = 0;
        let bytes = s.as_bytes(); // Using bytes is faster than chars() for ASCII

        // Iterate through the string in steps of 2
        for i in (0..bytes.len()).step_by(2) {
            // Check if the current pair (i, i+1) consists of different characters
            if bytes[i] != bytes[i + 1] {
                changes += 1;
            }
        }

        changes
    }
}

fn main() {}
