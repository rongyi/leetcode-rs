#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let s_doubled = s.clone() + &s;
        let s_chars: Vec<char> = s_doubled.chars().collect();

        // Create the two alternating patterns
        let mut alt1 = String::new(); // "0101..."
        let mut alt2 = String::new(); // "1010..."

        for i in 0..(2 * n) {
            if i % 2 == 0 {
                alt1.push('0');
                alt2.push('1');
            } else {
                alt1.push('1');
                alt2.push('0');
            }
        }

        let alt1_chars: Vec<char> = alt1.chars().collect();
        let alt2_chars: Vec<char> = alt2.chars().collect();

        let mut result = std::i32::MAX;
        let mut diff1 = 0; // Differences between s and alt1
        let mut diff2 = 0; // Differences between s and alt2

        // Initial window calculation (first n characters)
        for i in 0..n {
            if s_chars[i] != alt1_chars[i] {
                diff1 += 1;
            }
            if s_chars[i] != alt2_chars[i] {
                diff2 += 1;
            }
        }

        result = result.min(diff1.min(diff2));

        // Slide the window
        for i in 0..n {
            // Remove contribution of character going out of the window
            if s_chars[i] != alt1_chars[i] {
                diff1 -= 1;
            }
            if s_chars[i] != alt2_chars[i] {
                diff2 -= 1;
            }

            // Add contribution of character coming into the window
            if s_chars[i + n] != alt1_chars[i + n] {
                diff1 += 1;
            }
            if s_chars[i + n] != alt2_chars[i + n] {
                diff2 += 1;
            }

            // Update result
            result = result.min(diff1.min(diff2));
        }

        result
    }
}

fn main() {}
