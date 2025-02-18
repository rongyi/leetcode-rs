#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        // indicate the edge frog end at
        let mut counts = [0; 5]; // [c, r, o, a, k]
        let mut frogs = 0;
        let mut max_frogs = 0;

        for c in croak_of_frogs.chars() {
            match c {
                'c' => {
                    counts[0] += 1;
                    frogs += 1;
                    max_frogs = max_frogs.max(frogs);
                }
                'r' => {
                    if counts[0] == 0 {
                        return -1;
                    }
                    counts[0] -= 1;
                    // now a frog end at counts[1]
                    counts[1] += 1;
                }
                'o' => {
                    if counts[1] == 0 {
                        return -1;
                    }
                    counts[1] -= 1;
                    counts[2] += 1;
                }
                'a' => {
                    if counts[2] == 0 {
                        return -1;
                    }
                    counts[2] -= 1;
                    counts[3] += 1;
                }
                'k' => {
                    if counts[3] == 0 {
                        return -1;
                    }
                    counts[3] -= 1;
                    frogs -= 1;
                }
                _ => return -1, // Invalid character
            }
        }

        // Ensure all counts are zero (except 'k')
        if counts.iter().any(|v| *v != 0) {
            return -1;
        }

        max_frogs
    }
}

fn main() {}
