#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        // Create position arrays for each digit (0-9)
        let mut positions: Vec<Vec<usize>> = vec![vec![]; 10];

        // Store the positions of each digit in s
        for (i, &ch) in s_chars.iter().enumerate() {
            let digit = (ch as u8 - b'0') as usize;
            positions[digit].push(i);
        }

        // Initialize index pointers for each digit's position list
        let mut indices: Vec<usize> = vec![0; 10];

        // Process each digit in t
        for &ch in &t_chars {
            let digit = (ch as u8 - b'0') as usize;

            // Check if we've used all occurrences of this digit
            if indices[digit] >= positions[digit].len() {
                return false;
            }

            let pos = positions[digit][indices[digit]];

            // Check if there are smaller digits that come after this position
            for d in 0..digit {
                if indices[d] < positions[d].len() && positions[d][indices[d]] < pos {
                    return false;
                }
            }

            // Mark this position as used
            indices[digit] += 1;
        }

        true
    }
}

fn main() {}
